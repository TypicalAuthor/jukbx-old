<script lang="ts">
    import type { SongMetadata } from "../../types/fs-types";
    import type { updateRes } from "../../types/library-types";
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import ListView from "./list-view.svelte";
    import AlbumView from "./album-view.svelte";
    import anime from "animejs";

    export let view: "album" | "list";

    let local_library: SongMetadata[] = [];
    let unlisten: UnlistenFn;

    onMount(async () => {
        unlisten = await listen<updateRes>("library_updated", (res) => {
            local_library = res.payload.songArray;
        });
        await invoke<Array<SongMetadata>>("request_all_libraries").then(
            async (library_res) => {
                local_library = library_res;
                await emit("mainWebviewLoaded");
            },
        );
        anime({
            targets: ".no-entry-error",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "50%"],
            marginLeft: ["-25px", "0px"],
            duration: 400,
        });
    });

    onDestroy(() => {
        unlisten();
    });
</script>

<main>
    {#if local_library}
        {#if local_library.length == 0}
            <div class="no-entry-error">
                <h1>No songs detected!</h1>
                <p>
                    Please import a folder using the library manager, and make
                    sure the folder you imported has compatible files in it
                </p>
            </div>
        {:else if view == "album"}
            <AlbumView library={local_library}></AlbumView>
        {:else if view == "list"}
            <ListView library={local_library}></ListView>
        {/if}
    {:else}
        <div class="no-entry-error">
            <h1>No songs detected!</h1>
            <p>
                Please import a folder using the library manager, and make sure
                the folder you imported has compatible files in it
            </p>
        </div>
    {/if}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        overflow-y: auto;
        color: var(--text);
    }

    .no-entry-error {
        text-align: center;
        width: 500px;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        opacity: 0.5;
    }
</style>
