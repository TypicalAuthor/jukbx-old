<script lang="ts">
    import anime from "animejs";
    import { onDestroy, onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import type { Album } from "../../types/library-types";
    import { activeSong, setActiveSong } from "../../stores/song";
    import { isPlayerMini } from "../../stores/ui_state";
    import no_cover_alt from "../../assets/no_cover_alt.png";
    import type { SongMetadata } from "../../types/fs-types";
    import back from "../../assets/back.png";
    import { currentQueue } from "../../stores/queue";

    export let album: Album;

    let currentSong: SongMetadata;
    let local_isPlayerMini: boolean = false;

    const dispatch = createEventDispatcher();
    let anim: anime.AnimeInstance;

    const activeSongWatcherUnsub = activeSong.subscribe((song) => {
        currentSong = song;
    });

    const isPlayerMiniUnsub = isPlayerMini.subscribe((state) => {
        local_isPlayerMini = state;
    });

    onMount(() => {
        anime({
            targets: ".album-entries",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 500,
        });
        anime({
            targets: ".album-metadata-wrap",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 500,
        });
        anime({
            targets: ".album-song",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 500,
            delay: function (_a, i) {
                return i * 10;
            },
        });
    });

    function getAlbumImg(path: string) {
        if (path && path != "Some" && path != "None") {
            return convertFileSrc(path);
        }
        return no_cover_alt;
    }

    function outAnim() {
        anime.remove(".album-song .album-metadata-wrap .album-entries");
        anim = anime({
            targets: ".album-view-content-wrap",
            direction: "reverse",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            translateX: ["-25px", "0px"],
            duration: 350,
        });
    }

    function close() {
        outAnim();
        anim.finished.then(() => {
            dispatch("close");
        });
    }

    function handleQueue() {
        currentQueue.set({
            type: "album",
            contents: album.contents,
            position_song_id: album.contents[0].id,
            album_name: album.name,
            position_queue: 0,
        });
        activeSong.set(album.contents[0]);
    }

    function handleClickAlbum(i: number) {
        currentQueue.set({
            type: "album",
            contents: album.contents,
            position_song_id: i,
            album_name: album.name,
            position_queue: album.contents.findIndex((e) => e.id == i),
        });
    }

    onDestroy(() => {
        activeSongWatcherUnsub();
        isPlayerMiniUnsub();
    });
</script>

<main class="album-view-content-wrap">
    <div class="album-metadata-wrap">
        <!-- svelte-ignore a11y-missing-attribute -->
        <button class="album-close-btn" on:click={close}
            ><img src={back} width="50" height="50" /></button>
        {#if album.albumCoverPath != null}
            <img
                src={getAlbumImg(album.albumCoverPath)}
                alt="albumImg"
                width="490"
                height="490" />
        {:else}
            <img src={no_cover_alt} alt="albumImg" width="490" height="490" />
        {/if}
        <div class="album-options-wrap">
            <div class="album-metadata">{album.name}</div>
            <button class="album-play-btn" on:click={handleQueue}>Play</button>
        </div>
    </div>
    <div class="album-entries">
        {#each album.contents as song}
            {#if song != null}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div
                    class={"album-song" +
                        " " +
                        `${currentSong ? (currentSong.id == song.id ? "active-song" : "") : ""}`}
                    on:click={() => {
                        setActiveSong(song);
                        handleClickAlbum(song.id);
                    }}>
                    {song.title}
                    <div class="album-song-artist">
                        {`[${song.artist}]`}
                    </div>
                </div>
            {/if}
        {/each}
    </div>
</main>

<style>
    .album-song-artist {
        margin-left: 20px;
        font-size: smaller;
        opacity: 0.5;
    }
    .album-entries {
        border-left: 1px solid var(--accent);
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
        grid-template-rows: repeat(auto-fill, 40px);
        background-color: var(--secondary);
        height: 100%;
        overflow-y: auto;
        overflow-x: hidden;
    }
    .album-song {
        opacity: 0%;
        padding: 10px;
        height: 20px;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        padding-right: 10px;
        border: 1px solid rgba(255, 255, 255, 0.2);
        display: flex;
        align-items: center;
    }
    .album-song:hover {
        padding-right: 0px;
        cursor: pointer;
        padding-left: 25px;
        background-color: var(--deep-accent);
        color: var(--deep-accent-text);
    }

    .album-view-content-wrap {
        display: grid;
        grid-template-columns: 491px 1fr;
        height: 100%;
    }

    .album-close-btn {
        position: absolute;
        border: none;
        color: var(--text);
        background-color: var(--deep-accent);
        width: 60px;
        height: 60px;
        opacity: 0.9;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
    }

    .album-close-btn:hover {
        opacity: 1;
        cursor: pointer;
    }

    .active-song {
        background-color: var(--deep-accent);
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        padding-left: 50px;
        padding-right: 10px;
    }

    .active-song:hover {
        padding-right: 0px;
        background-color: var(--accent);
        color: var(--deep-accent);
        cursor: pointer;
        padding-left: 60px;
    }

    .album-metadata {
        display: flex;
        align-items: center;
        background-color: var(--secondary);
        padding-inline: 20px;
        font-size: larger;
        width: 100%;
        color: var(--text);
    }

    .album-metadata-wrap {
        color: black;
        z-index: 20;
        height: 585px;
    }

    .album-options-wrap {
        display: flex;
        justify-content: space-between;
        height: 94px;
    }

    .album-play-btn {
        aspect-ratio: 1/1;
        border: none;
        background-color: var(--deep-accent);
        color: var(--deep-accent-text);
        transition: 0.5s cubic-bezier(0.075, 0.82, 0.165, 1);
    }

    .album-play-btn:hover {
        background-color: var(--accent);
        color: var(--deep-accent);
        cursor: pointer;
    }
</style>
