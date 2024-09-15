<script lang="ts">
    import { xrSupported, enabled } from '$lib/store'
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import { init } from '../main';
    let ws: WebSocket
    class Stroke {
        constructor(points: number[][], color: number[]) {
            this.points = points
            this.color = color
        }
        points: number[][] = [];
        color: number[] = []
    }
    function sendCurve(curves: number[][]) {
        console.log(curves)
        ws.send(JSON.stringify([new Stroke(curves, [0, 0, 0])]))
    }
    onMount(() => {
        init(sendCurve)
        ws = new WebSocket('wss://canvas.nightland-smp.com:443/ws');
        if (ws) {
            ws.onopen = () => { 
                console.log("Yippee....")
            }
            ws.onerror = (event) => {
                console.log(event)
            }
        }
        
    })
    
</script>
<div class="absolute" id="dummy">
</div>
<div class="absolute top-0 left-0 w-[100vw] h-[100vh] flex">
    <div class="rounded-full bg-white border-black border-4 w-[40vw] h-[40vw] m-auto  data-[disabled]:opacity-0 data-[disabled]:pointer-events-none" data-disabled={null}/>
</div>

<style>
    
</style>