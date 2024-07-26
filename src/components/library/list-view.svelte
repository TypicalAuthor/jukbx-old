<script lang="ts">
    import { onMount } from "svelte";
    import type { SongMetadata } from "../../types/fs-types";
    import type { Album } from "../../types/library-types";
    import ListViewContent from "./list-view-content.svelte";
    import anime from "animejs";

    export let library: Array<SongMetadata>;

    onMount(() => {
        anime({
            targets: ".album-list",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 700,
        });
    });

    let albumRes: Array<Album> = [];
    let albumnames: Array<string> = [];
    let defaultAlbum: Array<SongMetadata> = [];
    let showGrid = true;

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
</script>

<main>
    {#if showGrid}
        <div class="album-list">
            {#each albumRes as entry}
                <ListViewContent album={entry}></ListViewContent>
            {/each}
        </div>
    {/if}
</main>

<style>
    .album-list {
        overflow-y: hidden;
    }
</style>
