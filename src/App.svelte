<script lang="ts">
    import type { SongMetadata } from "./types/fs-types";
    import { fileMenu, editMenu, helpMenu } from "./ts/menubar-items";
    import { onMount, onDestroy } from "svelte";
    import { emit } from "@tauri-apps/api/event";
    import {
        isPlayerActive,
        isPlayerMini,
        libraryView,
        showQueue,
    } from "./stores/ui_state";
    import Library from "./components/library/library.svelte";
    import Player from "./components/player/player.svelte";
    import Titlebar from "./components/titlebar.svelte";
    import Statusbar from "./components/statusbar.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { library_size } from "./stores/library";
    import LibraryOptions from "./components/library/library-options.svelte";
    import Queue from "./components/queue/queue.svelte";
    import anime from "animejs";

    let playerIsActive = false;
    let playerIsMini = false;
    let currentView: "list" | "album" = "album";
    let local_showQueue = false;

    let unsunLibraryView = libraryView.subscribe((view) => {
        currentView = view;
    });
    let unsubPlayerState = isPlayerActive.subscribe((e) => {
        playerIsActive = e;
    });
    let unsubPlayerIsMini = isPlayerMini.subscribe((e) => {
        playerIsMini = e;
    });
    let unsubQueueShow = showQueue.subscribe((e) => {
        if (e == false) {
            anime.remove(".queue-pos");
            anime({
                targets: ".queue-pos",
                direction: "normal",
                easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
                opacity: ["100%", "0%"],
                translateX: ["0px", "25px"],
                duration: 400,
            }).finished.then(() => {
                local_showQueue = e;
            });
        } else {
            local_showQueue = e;
        }
    });

    onMount(async () => {
        await invoke<Array<SongMetadata>>("request_library").then(async (res) => {
            library_size.set(res.length);
        });
        await emit("mainWebviewLoaded");
    });

    onDestroy(() => {
        unsubPlayerIsMini();
        unsubPlayerState();
        unsunLibraryView();
        unsubQueueShow();
    });
</script>

<main>
    <Titlebar
        windowName="JUKBX"
        menuBar={[fileMenu, editMenu, helpMenu]}
        hasTextBoxSearch={true}></Titlebar>
    <LibraryOptions></LibraryOptions>
    {#if local_showQueue}
        <Queue {playerIsActive} {playerIsMini}></Queue>
    {/if}
    <div
        class={playerIsActive
            ? playerIsMini
                ? "library-wrap-miniplayer"
                : "library-wrap-player"
            : "library-wrapper"}>
        <Library view={currentView}></Library>
    </div>

    <div class={`${playerIsActive ? "show" : "hide"}`}>
        <Player></Player>
    </div>

    <Statusbar></Statusbar>
</main>

<style>
    .library-wrap-miniplayer {
        height: calc(100vh - 165px);
    }
    .library-wrap-player {
        height: calc(100vh - 250px);
    }

    .library-wrapper {
        height: calc(100vh - 50px);
    }

    .hide {
        display: none;
    }

    .show {
        display: initial;
    }
</style>
