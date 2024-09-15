<script lang="ts">
    import { xrSupported, enabled } from '$lib/store'
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import { init } from '../main';
    let ws: WebSocket
    function sendCurve(curve: number[]) {
        
    }
    onMount(() => {
        init()
        ws = new WebSocket('wss://192.168.132.217/ws:8080');
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
    <div class="rounded-full bg-white border-black border-4 w-[40vw] h-[40vw] m-auto mb-2 z-[10000000000000000] data-[disabled]:opacity-0 data-[disabled]:pointer-events-none" data-disabled={false}/>
</div>
<style>
    
</style>