<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    let loaded: boolean = false;

    document.addEventListener("DOMContentLoaded", () => {
        loaded = true;
    });

    function closeSplashscreen() {
        invoke("show_window");
        return "loading...";
    }
</script>

<svelte:document on:contextmenu={(e) => e.preventDefault()} />

<main>
    <div class="splash-appTitle-location">
        <div class="splash-appTitle">JUKBX</div>
        <!-- svelte-ignore missing-declaration -->
        {#if loaded}
            {#await invoke("run_migrations")}
                <div class="splash-app-loadingStage">Loading database...</div>
            {:then a}
                <div class="splash-app-loadingStage">{closeSplashscreen()}</div>
            {:catch e}
                <div>ERROR WHILE RUNNING MIGRATIONS</div>
            {/await}
        {:else}
            <div class="splash-app-loadingStage">Loading...</div>
        {/if}
    </div>
</main>

<style>
    .splash-appTitle-location {
        position: absolute;
        top: 50vh;
        transform: translate(0%, -50%);
        padding-inline: 25px;
        display: flex;
        width: calc(100% - 52px);
        justify-content: space-between;
        align-items: center;
    }

    .splash-appTitle {
        color: var(--text);
        font-weight: bolder;
        font-size: larger;
    }

    .splash-app-loadingStage {
        text-align: left;
        color: var(--text);
        font-weight: lighter;
        font-size: smaller;
    }

    main {
        border: 1px solid var(--accent);
        width: calc(100vw - 2px);
        height: calc(100vh - 2px);
    }
</style>
