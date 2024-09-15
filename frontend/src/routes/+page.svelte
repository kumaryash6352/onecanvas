<script lang="ts">
    import { xrSupported, enabled } from '$lib/store'
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import { init } from '../main';
    import moment from 'moment';
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
<div class="absolute top-0 left-0 w-[100vw] h-[100vh] flex">
    <div class="rounded-full bg-white border-black border-4 w-[40vw] h-[40vw] m-auto  data-[disabled]:opacity-0 data-[disabled]:pointer-events-none" data-disabled={null}/>
</div>

<style>
    
</style>