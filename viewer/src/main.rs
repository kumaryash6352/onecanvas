use std::net::TcpStream;

use bevy::{prelude::*, render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages}};
use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};

#[derive(Resource, Debug, Deref)]
struct WsChannel(Receiver<Stroke>);

fn main() {
    println!("Hello, world!");

    let (socket, _response) =
        connect("wss://canvas.nightland-smp.com/ws").expect("to connect to ws");
    let (tx, rx) = crossbeam::channel::unbounded();
    std::thread::spawn(move || poll_ws(socket, tx));

    App::new()
        .add_event::<Stroke>()
        .insert_resource(WsChannel(rx))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, read_stream, spawn_strokes))
        .run();
}

fn spawn_strokes(
    mut commands: Commands,
    mut reader: EventReader<Stroke>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for stroke in reader.read() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(
                Mesh::new(
                    PrimitiveTopology::LineStrip,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, stroke.points.iter().flatten().map(|a| Vec3::from_array(*a)).collect::<Vec<_>>()),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_from_array(stroke.color),
                emissive: Color::srgb_from_array(stroke.color).into(),
                ..default()
            }),
            ..default()
        });
    }
}

fn rotate_camera(mut cameras: Query<&mut Transform, With<ViewerCamera>>, time: Res<Time>) {
    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };
    camera.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(Vec3::Y, time.delta_seconds() / 2.),
    );
}

fn read_stream(receiver: Res<WsChannel>, mut events: EventWriter<Stroke>) {
    for from_stream in receiver.try_iter() {
        events.send(from_stream);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(40., 40., 40.).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 4000.,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(30., 30., 20.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ViewerCamera,
    ));

}

fn poll_ws(mut socket: WebSocket<MaybeTlsStream<TcpStream>>, tx: Sender<Stroke>) {
    'ws: loop {
        let Ok(msg) = socket.read() else {
            break;
        };
        let Ok(txt) = msg.into_text() else {
            continue;
        };

        let to_add = if txt.starts_with('[') {
            let Ok(strokes) = serde_json::from_str(&txt) else {
                continue;
            };
            strokes
        } else {
            let Ok(stroke) = serde_json::from_str(&txt) else {
                continue;
            };
            vec![stroke]
        };

        for stroke in to_add {
            let Ok(_) = tx.send(stroke) else {
                break 'ws;
            };
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
struct Stroke {
    points: Vec<Vec<[f32; 3]>>,
    color: [f32; 3],
}

#[derive(Component, Copy, Clone)]
struct ViewerCamera;
