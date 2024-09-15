import { ARButton } from 'three/addons/webxr/ARButton.js';
let camera, scene, renderer;
let controller;
let defaultEnvironment;
let state = 0
let points = []
let curves = [[]]
let drawingLate = false
import { XRGestures } from '$lib/XRGestures';
import * as THREE from "three"
import * as M from "three.meshline"
import { drawing } from '$lib/store'
export function init(func) {

    const container = document.createElement( 'div' );
    const grabby = document.getElementById("dummy")
    grabby.appendChild( container );

    scene = new THREE.Scene();

    camera = new THREE.PerspectiveCamera( 70, window.innerWidth / window.innerHeight, 0.01, 20 );

    const curve = new THREE.QuadraticBezierCurve3(
        new THREE.Vector3( 0, 0, 0 ),
        new THREE.Vector3( 1, 0, 0 ),
        new THREE.Vector3( 0, 0, 1 )
    );

    const points = curve.getPoints( 50 );
    const geometry = new THREE.BufferGeometry().setFromPoints( points );

    const material = new THREE.LineBasicMaterial( { color: 0xff0000, linewidth: 8 } );

    const curveObject = new THREE.Line( geometry, material );

    scene.add(curveObject);
    const defaultLight = new THREE.HemisphereLight( 0xffffff, 0xbbbbff, 1 );
    // defaultLight.position.set( 0.5, 1, 0.25 );
    // scene.add( defaultLight );

    //
    function select() {
        console.log("SELECT")
    }
    renderer = new THREE.WebGLRenderer( { antialias: true, alpha: true } );
    renderer.setPixelRatio( window.devicePixelRatio );
    renderer.setSize( window.innerWidth, window.innerHeight );
    renderer.setAnimationLoop( animate );
    renderer.xr.enabled = true;
    container.appendChild( renderer.domElement );
    
    // Don't add the XREstimatedLight to the scene initially.
    // It doesn't have any estimated lighting values until an AR session starts.

    let button = ARButton.createButton( renderer, { domOverlay: {root: document.body} } ) 
    button.classList.add("button")
    // In order for lighting estimation to work, 'light-estimation' must be included as either an optional or required feature.
    document.body.appendChild( button );
    let touchListen = new XRGestures(renderer)

    window.addEventListener( 'resize', onWindowResize );
    touchListen.addEventListener("tap", () => {
        console.log("START")
        drawing = true
        drawingLate = true
    })
    touchListen.addEventListener("tap", () => {
        console.log("WHOA")
        drawing = false
    })
}

function onWindowResize() {

    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );

}

//

function animate() {
    
    renderer.render( scene, camera );
    if (!drawing) {
        state = 0;
        if (drawingLate) {
            drawingLate = false;
            func(curves)
        }
        return;
    }
    switch(state) {
        case 0:
            points[0] = camera.position;
            break;
        case 1:
            points[1] = camera.position;
            break;
        case 2:
            points[2] = camera.position;
            curves.push(points)
            points[0] = camera.position;
            state = 0;
    }
    state++
}