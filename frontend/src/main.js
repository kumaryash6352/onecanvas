import { ARButton } from 'three/addons/webxr/ARButton.js';
let camera, scene, renderer;
let controller;
let defaultEnvironment;
let state = 0
let points = []
let curves = []
let drawing = false
let drawingLate = false
import { XRGestures } from '$lib/XRGestures';
import * as THREE from "three"
import * as M from "three.meshline"
export function init() {

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
    // defaultLight.position.set( 0.5, 1, 0.25 );
    // scene.add( defaultLight );

    //

    globalThis.addCurve = (curve) => {
        curve['points'].forEach(element => {
            let temp = []   

            element.forEach(e => {
                temp.push(new THREE.Vector3().fromArray(e))
            })
            console.log(temp)
            scene.add(new THREE.Line(
                new THREE.BufferGeometry().setFromPoints(new THREE.QuadraticBezierCurve3(temp[0], temp[1], temp[2]).getPoints(5)),
                new THREE.LineBasicMaterial( { 
                    color: Math.floor(curve['color'][0] * 255) * 0x10000
                        + Math.floor(curve['color'][1] * 255) * 0x100
                        + Math.floor(curve['color'][2] * 255) * 0x1,
                    linewidth: 8 
                } )
            ))
        });
        
    }
    renderer = new THREE.WebGLRenderer( { antialias: true, alpha: true } );
    renderer.setPixelRatio( window.devicePixelRatio );
    renderer.setSize( window.innerWidth, window.innerHeight );
    renderer.setAnimationLoop( animate );
    renderer.xr.enabled = true;
    container.appendChild( renderer.domElement );
    
    // Don't add the XREstimatedLight to the scene initially.
    // It doesn't have any estimated lighting values until an AR session starts.

    let button = ARButton.createButton( renderer, { } ) 
    button.classList.add("button")
    // In order for lighting estimation to work, 'light-estimation' must be included as either an optional or required feature.
    document.body.appendChild( button );
    let touchListen = new XRGestures(renderer)

    window.addEventListener( 'resize', onWindowResize );
    globalThis.selectstart = () => {
        drawing = true
        drawingLate = true
    }
    globalThis.selectend = () => {
        drawing = false
    }
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
            globalThis.sendCurve(curves)
        }
        return;
    }
    switch(state) {
        case 0:
            points[0] = [...camera.position];
            break;
        case 1:
            points[1] = [...camera.position];
            break;
        case 2:
            points[2] = [...camera.position];
            curves.push([...points])
            points[0] = [...camera.position];
            state = 0;
    }
    state++
}