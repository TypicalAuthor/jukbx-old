<script lang="ts">
    import type { SongMetadata } from "../../types/fs-types";
    import type { Album } from "../../types/library-types";
    import { isPlayerActive, isPlayerMini } from "../../stores/ui_state";
    import AlbumViewCard from "./album-view-card.svelte";
    import AlbumViewContent from "./album-view-content.svelte";
    import anime from "animejs";
    import { onDestroy } from "svelte";

    export let library: Array<SongMetadata>;

    let anim: anime.AnimeInstance;
    let local_isPlayerMini: boolean = false;
    let local_isPlayerActive: boolean = false;

    const isPlayerActiveUnsub = isPlayerActive.subscribe((state) => {
        local_isPlayerActive = state;
    });

    let isPlayerMiniUnsub = isPlayerMini.subscribe((state) => {
        local_isPlayerMini = state;
    });

    function outAnim() {
        anime.remove(".song-library-card");
        anim = anime({
            targets: ".song-library-card",
            direction: "reverse",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 350,
        });
    }

    let albumRes: Array<Album> = [];
    let albumnames: Array<string> = [];
    let defaultAlbum: Array<SongMetadata> = [];
    let showGrid = true;
    let album: Album;

    for (let i = 0; library.length > i; i++) {
        if (library[i].album != null) {
            albumnames.push(library[i].album!);
        } else {
            defaultAlbum.push(library[i]);
        }
    }

    albumnames = Array.from(new Set(albumnames));
    for (let i = 0; albumnames.length > i; i++) {
        const res = library
            .filter((a) => a.album == albumnames[i])
            .sort((a, b) => Number(a.track_number) - Number(b.track_number));
        const tempAlb: Album = {
            name: albumnames[i],
            albumCoverPath: res[0].image_path == null ? "Some" : res[0].image_path,
            contents: res,
        };
        albumRes.push(tempAlb);
    }
    if (defaultAlbum.length != 0) {
        albumRes.push({
            name: "Unknown album",
            contents: defaultAlbum,
        });
    }

    function showAlbum() {
        showGrid = !showGrid;
    }

    function updateAlbum(e: CustomEvent<{ album: Album }>) {
        outAnim();
        anim.finished.then(() => {
            album = e.detail.album;
            showAlbum();
        });
    }

    onDestroy(() => {
        isPlayerMiniUnsub();
        isPlayerActiveUnsub();
    });
</script>

<main>
    {#if !showGrid && album != undefined}
        <div
            class={local_isPlayerActive && !local_isPlayerMini
                ? "relative-wrap-player"
                : local_isPlayerMini
                  ? "relative-wrap-miniplayer"
                  : "relative-wrap"}>
            <AlbumViewContent on:close={showAlbum} {album}></AlbumViewContent>
        </div>
    {/if}

    {#if showGrid}
        <div class="album-grid">
            {#each albumRes as entry}
                <AlbumViewCard {entry} on:updateAlbum={updateAlbum}></AlbumViewCard>
            {/each}
        </div>
    {/if}
</main>

<style>
    .album-grid {
        padding: var(--padding-inline);
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
        gap: 10px;
    }

    .relative-wrap {
        width: 100%;
        height: calc(100vh - 85px);
        overflow-y: auto;
        color: var(--text);
        position: relative;
        overflow: hidden;
    }

    .relative-wrap-player {
        width: 100%;
        height: calc(100vh - 250px);
        overflow-y: auto;
        color: var(--text);
        position: relative;
        overflow: hidden;
    }

    .relative-wrap-miniplayer {
        width: 100%;
        height: calc(100vh - 165px);
        overflow-y: auto;
        color: var(--text);
        position: relative;
        overflow: hidden;
    }
</style>
