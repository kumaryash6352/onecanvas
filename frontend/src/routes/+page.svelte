<script lang="ts">
    import { xrSupported, enabled } from '$lib/store'
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import { init } from '../main';
    let colors = [
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
        [1, 1, 0],
        [1, 0, 1],
        [0, 1, 1],
        [1, 1, 1],
        [.17, .08, 0]
    ]
    let ws: WebSocket
    class Stroke {
        constructor(points: number[][], color: number[]) {
            this.points = points
            this.color = color
        }
        points: number[][];
        color: number[]
    }
    (globalThis as any).sendCurve = (curves: number[][]) => {
        //ws.send(JSON.stringify(new Stroke(curves, colors[time%colors.length])))
        ws.send(JSON.stringify({
            points: curves,
            color: colors[Math.floor(time)%colors.length]
        }))
    }
    let time = 0
    onMount(() => {
        init()
        time = window.performance.now()
        ws = new WebSocket('wss://canvas.nightland-smp.com:443/ws');
        
        ws.onopen = (message) => { 
            console.log("Yippee....")
            
        }
        ws.onmessage = (message) => {
            let data = JSON.parse(message.data as string)
            console.log(data)
            if (Array.isArray(data))
                data.forEach((element: {}) => { 
                    (globalThis as any).addCurve(element)
                })
            else
                (globalThis as any).addCurve(data)
            
        }
        ws.onerror = (event) => {
            console.log(event)
        }
        
    })
    
</script>
<div class="absolute" id="dummy">
</div>
<div class="absolute w-[100vw] h-[100vh] flex flex-col">
    <div class="m-auto text-[13vw] mb-2 font-black mt-[30vh]">Onecanvas</div>
    <div class="m-auto mt-2">Click the button below to center your phone</div>
</div>
<style>
    
</style>