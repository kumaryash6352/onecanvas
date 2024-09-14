use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::extract::ws::WebSocket;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::get;
use axum::debug_handler;
use axum::Router;
use tokio::net::TcpListener;
use tokio::select;
use glam::Vec3;
use log::info;
use log::trace;
use futures::*;
use mongodb::bson::Document;
use mongodb::options::ServerApi;
use mongodb::options::ServerApiVersion;
use mongodb::Client;
use mongodb::Cursor;
use mongodb::{bson::doc, options::ClientOptions};
use serde::Deserialize;
use serde::Serialize;
use tokio::spawn;
use tokio::sync::broadcast::channel;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

fn env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

#[derive(Debug)]
struct AppState {
    strokes: Vec<Stroke>,
    stroke_rx: Receiver<Stroke>,
    stroke_tx: Sender<Stroke>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stroke {
    points: Vec<(Vec3, Vec3, Vec3)>,
    color: [f32; 3],
    owner: usize,
    id: usize
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mongoname = env("MONGO_DB_USERNAME").expect("need MONGO_DB_USERNAME");
    let mongopass = env("MONGO_DB_PASSWORD").expect("need MONGO_DB_PASSWORD");

    trace!("Initiating MongoDB connection...");
    let mut client_options = ClientOptions::parse(
        format!("mongodb+srv://{mongoname}:{mongopass}@onecanvas.0svzr.mongodb.net/?retryWrites=true&w=majority&appName=OneCanvas")).await
        .context("connecting to mongodb")?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;
    info!("Successfully connected to MongoDB.");

    info!("Pulling latest point data from database...");
    let strokes_cursor: Cursor<_> = client.database("onecanvas")
        .collection::<Stroke>("strokes")
        .find(doc! {})
        .await?;
    let strokes = strokes_cursor.try_collect::<Vec<_>>().await?;
    info!("Restored {} strokes.", strokes.len());

    info!("Preparing Axum server...");

    let (stroke_tx, stroke_rx) = channel(64);
    let state = AppState {
        strokes,
        stroke_rx: stroke_rx.resubscribe(),
        stroke_tx: stroke_tx.clone(),
    };

    let state = Arc::new(RwLock::from(state));
    spawn(stroke_collector(stroke_rx, Arc::clone(&state)));



    let app = Router::new()
        .route("/", get(page))
        .route("/ws", get(handler))
        .with_state(state)
        ;

    let nic = env("HOST_NIC").expect("HOST_NIC to be set");
    let port = env("HOST_PORT").expect("HOST_PORT to be set");
    let listener = TcpListener::bind(format!("{nic}:{port}")).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[debug_handler]
async fn page() -> &'static str { " hello " }

#[debug_handler]
async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let (rx, tx) = {
        let lock = state.read().await;
        (lock.stroke_rx.resubscribe(), lock.stroke_tx.clone())
    };
    let state_handle = Arc::clone(&state);
    ws.on_upgrade(move |mut socket| async move {
        // send initial data
        let lock = state_handle.read().await;
        socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&lock.strokes).expect("strokes to ser"))).await.ok();
        websocketer(rx, tx, socket).await.ok();
    })
}

async fn websocketer(mut rx: Receiver<Stroke>, tx: Sender<Stroke>, mut ws: WebSocket) -> Result<()> {
    loop {
        select! {
            Ok(s) = rx.recv() => ws.send(axum::extract::ws::Message::Text(serde_json::to_string(&s).expect("stroke to always serialize"))).await?,
            m = ws.recv() =>  {
                if let Some(Ok(m)) = m {
                    let maybe_stroke = serde_json::from_slice::<Stroke>(&m.into_data());
                    if let Ok(stroke) = maybe_stroke {
                        tx.send(stroke).expect("server to be live");
                    }
                } else {
                    break Ok(())
                }
            }
        }
    }
}

async fn stroke_collector(rx: Receiver<Stroke>, state: Arc<RwLock<AppState>>) {
    const SLEEP: Duration = Duration::from_secs(20);
    let mut stream = BroadcastStream::new(rx);
    loop {
        sleep(SLEEP).await;
        let mut lock = state.write().await;
        while let Some(Ok(stroke)) = stream.next().await {
            lock.strokes.push(stroke);
        }
        drop(lock);
    }
}