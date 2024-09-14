<script lang="ts">
    import { xrSupported } from '$lib/store'
    import { onMount } from 'svelte';
    import { T, Canvas,  } from "@threlte/core"
    import { XR , } from '@threlte/xr'
    import { OrbitControls, Text, MeshLineGeometry, MeshLineMaterial, } from '@threlte/extras'
    import { ARButton, XRButton } from '@threlte/xr'
    import * as THREE from 'three'
    onMount(() => {
        
    })
    async function start(session: XRSession) {
        
    }
    const curve = new THREE.QuadraticBezierCurve3(
        new THREE.Vector3(0, 0, 0),
        new THREE.Vector3(1, 0, 0),
        new THREE.Vector3(0, 1, 0),
    )
    const points = curve.getPoints(50)
</script>
<XRButton
  sessionInit={{
    domOverlay: typeof document !== 'undefined' ? { root: document.body } : undefined,
    requiredFeatures: [],
    optionalFeatures: ['local-floor', 'bounded-floor', 'hand-tracking', 'layers', 'hit-test', 'plane-detection', 'image-tracking'],
    
  
  }}
  {...$$restProps}
  mode="immersive-ar"
  on:click
  on:error
/>
<Canvas >
    
    <XR foveation={1} on:sessionstart={(event) => {start(event.target);}}>
        <T.PerspectiveCamera >
            <OrbitControls />     
        </T.PerspectiveCamera>
        
        <T.Mesh position={[0, 1.6, -1]}>
            <MeshLineGeometry {points} shape='taper'/>
            <MeshLineMaterial color={0xffffff}/>
        </T.Mesh>

        <Text
            position={[0, 1.6, -1]}
            text="aaaa"
        />
    </XR>
</Canvas>
<style>
    
</style>