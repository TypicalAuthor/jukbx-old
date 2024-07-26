<script lang="ts">
    import type { Album } from "../../types/library-types";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import anime from "animejs";
    import no_cover_alt from "../../assets/no_cover_alt.png";

    export let entry: Album;

    const dispatch = createEventDispatcher();
    let isOpen = false;

    function forwardAlbum() {
        dispatch("updateAlbum", {
            album: entry,
        });
    }

    onMount(() => {
        anime({
            targets: ".song-library-card",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 450,
            delay: function (_el, i) {
                return i * 5;
            },
        });
    });

    let album_img: HTMLImageElement;

    function getAlbumImg(path: string) {
        if (path && path != "Some" && path != "None") {
            return convertFileSrc(path);
        }
        return no_cover_alt;
    }

    function handleError() {
        if (album_img) {
            album_img.src = no_cover_alt;
        }
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-to-interactive-role -->
<main
    class="song-library-card"
    role="button"
    tabindex="0"
    on:click={() => (isOpen = !isOpen)}
    on:click={forwardAlbum}>
    {#if entry.albumCoverPath != null}
        <img
            alt="albumcover"
            src={getAlbumImg(entry.albumCoverPath)}
            width="123"
            height="123"
            loading="lazy"
            bind:this={album_img}
            on:error={handleError} />
    {:else}
        <img
            alt="albumcover"
            src={no_cover_alt}
            width="123"
            height="123"
            loading="lazy"
            bind:this={album_img} />
    {/if}
    {#if entry.name != null && entry.name.trim() != ""}
        <div class="album-title">{entry.name}</div>
    {:else}
        <div class="album-title">Unknown Album</div>
    {/if}
</main>

<style>
    .song-library-card {
        opacity: 0%;
        border: 1px solid var(--accent);
        background-color: var(--secondary);
        text-align: center;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        transform: translateX(-25px);
        min-width: 123px;
        max-width: 123px;
        max-height: 160px;
        min-height: 160px;
    }

    .song-library-card:hover {
        background-color: var(--primary);
        color: black;
        cursor: pointer;
    }

    .album-title {
        font-size: small;
        padding: 5px;
        padding-inline: 10px;
        overflow-x: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
    }
</style>
