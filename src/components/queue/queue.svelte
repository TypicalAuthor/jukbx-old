<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { currentQueue } from "../../stores/queue";
    import type { Queue } from "../../types/queue-types";
    import { activeSong, setActiveSong } from "../../stores/song";
    import type { SongMetadata } from "../../types/fs-types";
    import anime from "animejs";
    import { showQueue } from "../../stores/ui_state";

    let local_currentQueue: Queue;
    let local_activeSong: SongMetadata;
    export let playerIsActive: boolean;
    export let playerIsMini: boolean;

    onMount(() => {
        anime({
            targets: ".queue-pos",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["25px", "0px"],
            duration: 400,
        });
    });

    const unsubCurrentQueue = currentQueue.subscribe((queue) => {
        local_currentQueue = queue;
    });

    const unsubActiveSong = activeSong.subscribe((song) => {
        local_activeSong = song;
    });

    function handleClick(queueEntry: SongMetadata) {
        currentQueue.set({
            type: local_currentQueue.type,
            contents: local_currentQueue.contents,
            position_song_id: queueEntry.id,
            album_name: local_currentQueue.album_name,
            position_queue: local_currentQueue.contents.findIndex(
                (e) => e.id == queueEntry.id,
            ),
        });
        setActiveSong(queueEntry);
    }

    function handleClickOut(
        e: MouseEvent & { currentTarget: EventTarget & HTMLElement } & {
            target: HTMLElement;
        },
    ) {
        if (e.target) {
            if (e.target.classList[0] != undefined) {
                if (!e.target.classList[0].toString().startsWith("queue-")) {
                    showQueue.set(false);
                }
            } else {
                showQueue.set(false);
            }
        }
    }

    onDestroy(() => {
        unsubCurrentQueue();
        unsubActiveSong();
    });
</script>

<svelte:body
    on:mousedown={(e) => {
        //@ts-ignore
        handleClickOut(e);
    }} />

<main
    class={`queue-pos ${
        playerIsActive
            ? playerIsMini
                ? "queue-wrap-miniplayer"
                : "queue-wrap-player"
            : "queue-wrapper"
    }`}>
    <div class="queue-container">
        <div class="queue-position-wrap">
            {#if local_currentQueue}
                <div class="queue-metadata-wrap">
                    <div class="queue-search-type">
                        Playing from {local_currentQueue.type}
                    </div>
                    {#if local_currentQueue.album_name}
                        <div class="queue-name">{local_currentQueue.album_name}</div>
                    {/if}
                </div>
                <div class="queue-content-wrap">
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    {#each local_currentQueue.contents as queueEntry}
                        <!-- svelte-ignore a11y-click-events-have-key-events -->
                        <div
                            class={"queue-entry" +
                                " " +
                                `${local_activeSong ? (local_activeSong.id == queueEntry.id ? "queue-active-entry" : "") : ""}`}
                            on:click={() => handleClick(queueEntry)}>
                            <div class="queue-entry-title">{queueEntry.title}</div>
                            <div class="queue-entry-artist">
                                {`[${queueEntry.artist}]`}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</main>

<style>
    .queue-pos {
        position: absolute;
        z-index: 10;
        right: 0;
    }
    .queue-wrap-miniplayer {
        height: calc(100vh - 166px);
    }
    .queue-wrap-player {
        height: calc(100vh - 251px);
    }
    .queue-wrapper {
        height: calc(100vh - 50px);
    }
    .queue-search-type {
        opacity: 0.5;
        font-size: 0.75rem;
    }
    .queue-entry {
        display: flex;
        padding-top: 2px;
        padding-bottom: 5px;
        height: 15px;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        padding-right: 10px;
        color: white;
        font-size: smaller;
    }

    .queue-content-wrap > :global(:not(:last-child)) {
        border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    }
    .queue-entry:hover {
        padding-right: 0px;
        cursor: pointer;
        padding-left: 25px;
        background-color: var(--deep-accent);
        color: var(--deep-accent-text);
    }

    .queue-active-entry {
        background-color: var(--deep-accent);
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        padding-left: 50px;
        padding-right: 10px;
        color: var(--accent);
        font-weight: bold;
    }

    .queue-active-entry:hover {
        padding-right: 0px;
        background-color: var(--accent);
        color: var(--deep-accent);
        cursor: pointer;
        padding-left: 60px;
    }
    .queue-entry-artist {
        opacity: 0.5;
        font-size: smaller;
        margin-left: 20px;
    }
    .queue-entry {
        display: flex;
        align-items: center;
        padding-top: 5px;
        padding-bottom: 5px;
    }
    .queue-content-wrap {
        color: white;
        padding-inline: var(--padding-inline);
    }
    .queue-metadata-wrap {
        background-color: var(--secondary);
        padding: var(--padding-inline);
        color: var(--deep-accent-text);
    }
    main {
        height: 100%;
        right: 0;
        width: 500px;
        max-width: 500px;
        overflow-y: auto;
        overflow-x: hidden;
        border: 1px solid var(--accent);
        border-right: none;
        border-bottom: none;
        background-color: var(--deep-accent);
        border-left: 1px solid var(--accent);
        box-shadow: 0px 0px 5px 1px rgba(0, 0, 0, 0.5);
        opacity: 0;
    }
</style>
