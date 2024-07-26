<script lang="ts">
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { library_size } from "../stores/library";
    import { activeSong } from "../stores/song";
    import { onDestroy, onMount } from "svelte";

    let local_library_size: number;
    let activeSong_title: string | null;
    let isProccesing = false;
    let unlistenProccessing: UnlistenFn;
    let unlistenFinished: UnlistenFn;

    onMount(() => {
        generateListeners();
    });

    async function generateListeners() {
        unlistenProccessing = await listen("processing_file", () => {
            isProccesing = true;
        });

        unlistenFinished = await listen("proccessing_finished", () => {
            isProccesing = false;
        });
    }
    let librarySizeUnsub = library_size.subscribe((size) => {
        local_library_size = size;
    });
    let current_songUnsub = activeSong.subscribe((song) => {
        if (song) {
            activeSong_title = song.title;
        }
    });

    onDestroy(() => {
        librarySizeUnsub();
        current_songUnsub();
        if (unlistenProccessing) {
            unlistenProccessing();
        }
        if (unlistenFinished) {
            unlistenFinished();
        }
    });
</script>

<main class="statusbar-wrap">
    <div class="left">
        {#if isProccesing}
            <div>Importing files...</div>
        {/if}
    </div>

    <div class="center">
        {#if activeSong_title}
            <div>Now listening: {activeSong_title}</div>
        {/if}
    </div>

    <div class="right">
        {#if local_library_size}
            <div>Files: {local_library_size}</div>
        {/if}
    </div>
</main>

<style>
    .statusbar-wrap {
        height: 20px;
        border-top: 1px solid var(--accent);
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        justify-content: space-between;
        padding-inline: var(--padding-inline);
        background-color: var(--secondary);
        color: rgba(255, 255, 255, 0.5);
        font-size: smaller;
    }
    .center {
        text-align: center;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        max-width: 520px;
    }
    .left {
        text-align: left;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        max-width: 200px;
    }
    .right {
        text-align: right;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        max-width: 200px;
    }
</style>
