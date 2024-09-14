<script lang="ts">
    import "../app.css";
    import { onMount } from "svelte";
    onMount(() => {
        
        
    })
</script>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div style="width: 100vw; height: 100vh">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="absolute left-2 top-2 p-2 bg-slate-300 rounded-xl cursor-pointer select-none hover:bg-slate-600 hover:text-white transition-all"
        on:click={() => {
            if (document.fullscreenElement != null) 
                document.exitFullscreen()
            else
                document.documentElement.requestFullscreen();
        }}>
        Fullscreen
    </div>
    <slot />
</div>
