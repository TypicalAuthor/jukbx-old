<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import type { SongMetadata } from "../../types/fs-types";
    import no_cover_alt from "../../assets/no_cover_alt.png";
    import { activeSong } from "../../stores/song";
    import { onMount } from "svelte";
    import anime from "animejs";
    import { currentQueue } from "../../stores/queue";

    export let result: Array<SongMetadata> = [];

    export let shouldShowResBox: boolean;

    onMount(() => {
        anime({
            targets: ".resbox-wrap",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateY: ["25px", "0px"],
            duration: 500,
        });
        anime({
            targets: ".result-wrapper",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateY: ["25px", "0px"],
            duration: 500,
            delay: function (_el, i) {
                return i * 20;
            },
        });
    });

    function getAlbumImg(path: string | null) {
        if (path && path != "Some" && path != "None") {
            return convertFileSrc(path);
        }
        return no_cover_alt;
    }

    function handleClick(id: number) {
        if (result[id]) {
            activeSong.set(result[id]);
            currentQueue.set({
                type: "search",
                contents: result,
                position_song_id: id,
                position_queue: result.findIndex((e) => e.id == id),
            });
            shouldShowResBox = false;
        }
    }
</script>

<main class="resbox-wrap">
    {#each result as foundSong, i}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="result-wrapper" on:click={() => handleClick(i)}>
            <div class="result-img">
                <!-- svelte-ignore a11y-missing-attribute -->
                <img
                    src={getAlbumImg(foundSong.image_path)}
                    height="50"
                    width="50"
                    class="result-img" />
            </div>
            <div class="metadata-wrapper">
                {#if foundSong.title}
                    <div class="result-title">{foundSong.title}</div>
                {:else}
                    <div class="result-title">Unknown title</div>
                {/if}
                {#if foundSong.artist}
                    <div class="result-artist">{foundSong.artist}</div>
                {:else}
                    <div class="result-artist">Unknown artist</div>
                {/if}
            </div>
        </div>
    {/each}
</main>

<style>
    .result-artist {
        font-weight: lighter;
        font-size: small;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        width: 320px;
    }
    .result-title {
        font-weight: bold;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        width: 330px;
    }
    .result-wrapper {
        display: flex;
        padding: 5px;
        padding-bottom: 10px;
        padding-top: 10px;
        gap: 5px;
        align-items: center;
        transition: 0.5s cubic-bezier(0.075, 0.82, 0.165, 1);
        opacity: 0;
    }

    .result-wrapper:hover {
        background-color: var(--deep-accent);
        cursor: pointer;
    }
    .resbox-wrap {
        font-size: small;
        position: absolute;
        top: 32px;
        border: 1px solid var(--deep-accent);
        width: 400px;
        right: 20px;
        min-height: 70px;
        z-index: 1000000;
        background-color: var(--secondary);
        box-shadow: 0px 0px 10px 1px rgba(0, 0, 0, 0.5);
    }

    .resbox-wrap > :global(:not(:last-child)) {
        border-bottom: 1px solid rgba(101, 185, 200, 0.2);
    }
</style>
