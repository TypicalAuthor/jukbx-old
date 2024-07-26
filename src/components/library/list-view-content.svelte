<script lang="ts">
    import { onDestroy } from "svelte";
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import type { Album } from "../../types/library-types";
    import { activeSong, setActiveSong } from "../../stores/song";
    import no_cover_alt from "../../assets/no_cover_alt.png";
    import type { SongMetadata } from "../../types/fs-types";
    import { currentQueue } from "../../stores/queue";
    import play from "../../assets/play.png";

    export let album: Album;
    let currentSong: SongMetadata;
    let album_bg_element: HTMLElement;

    const activeSongWatcherUnsub = activeSong.subscribe((song) => {
        currentSong = song;
    });

    function getAlbumImg(path: string | undefined) {
        if (path && path != "Some" && path != "None") {
            return convertFileSrc(path);
        }
        return no_cover_alt;
    }

    async function handleClickLibrary(i: number) {
        let res = await invoke<Array<SongMetadata>>(
            "request_all_libraries",
        ).then(async (library_res) => {
            return library_res;
        });
        //WARNING: THIS FUNCTION MIGHT BE REALLY SLOW TO SEARCH, PROBABLY A BETTER OPTION IS TO FILTER FIRST BY ALBUM AND THEN BY ID
        currentQueue.set({
            type: "library",
            contents: res,
            position_song_id: i,
            position_queue: res.findIndex((e) => e.id == i),
        });
    }

    function handleClickAlbum() {
        currentQueue.set({
            type: "album",
            contents: album.contents,
            position_song_id: album.contents[0].id,
            album_name: album.name,
            position_queue: 0,
        });
        activeSong.set(album.contents[0]);
    }

    onDestroy(() => {
        activeSongWatcherUnsub();
    });
</script>

<main class="album-view-cover" bind:this={album_bg_element}>
    <!-- svelte-ignore a11y-missing-attribute -->
    <img src={getAlbumImg(album.albumCoverPath)} class="album-background" />
    <div class="album-view-content-wrap">
        <div class="album-view-layout">
            <div class="album-img-position-wrap">
                <div class="album-img-wrap">
                    {#if album.albumCoverPath != null}
                        <img
                            src={getAlbumImg(album.albumCoverPath)}
                            alt="albumImg"
                            width="180"
                            height="180" />
                    {:else}
                        <img
                            src={no_cover_alt}
                            alt="albumImg"
                            width="180"
                            height="180" />
                    {/if}
                </div>
            </div>
            <div class="right">
                <div class="album-name">
                    <div class="album-metadata">
                        <button
                            on:click={handleClickAlbum}
                            class="album-play-btn">
                            <!-- svelte-ignore a11y-missing-attribute -->
                            <img src={play} height="20" width="20" /></button>
                        <div class="album-title">{album.name}</div>
                        <div class="album-metadata-small">
                            <div class="divider">|</div>
                            <div class="album-size">
                                {album.contents.length}
                                {album.contents.length != 1
                                    ? "entries"
                                    : "entry"}
                            </div>
                        </div>
                    </div>
                </div>
                <div class="album-entries">
                    {#each album.contents as song, i}
                        {#if song != null}
                            <!-- svelte-ignore a11y-no-static-element-interactions -->
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            <div
                                class={"album-song" +
                                    " " +
                                    `${currentSong ? (currentSong.id == song.id ? "active-song" : "") : ""}`}
                                on:click={() => {
                                    setActiveSong(song);
                                    handleClickLibrary(song.id);
                                }}>
                                {`${i + 1}. `}
                                {song.title}
                                <div class="album-song-artist">
                                    {`[${song.artist}]`}
                                </div>
                            </div>
                        {/if}
                    {/each}
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    .album-view-cover {
        position: relative;
        height: 100%;
        width: 100%;
    }
    .album-song-artist {
        color: var(--deep-accent-text);
        margin-left: 20px;
    }
    .album-title {
        width: calc(100vw - 500px);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
    }
    .divider {
        margin-inline: 10px;
    }
    .album-metadata-small {
        align-items: center;
        display: flex;
        font-size: small;
        font-weight: lighter;
        opacity: 0.5;
        width: 100px;
    }
    .album-view-layout {
        display: flex;
        width: 100%;
        height: 100%;
    }
    .album-background {
        position: absolute;
        width: 100%;
        height: 100%;
        overflow: hidden;
    }
    .album-img-position-wrap {
        position: relative;
        height: 200px;
        width: 200px;
    }
    .album-entries {
        width: calc(100vw - 200px - 20px);
        overflow-y: auto;
        overflow-x: hidden;
    }
    .album-song {
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

    .album-entries > :global(:not(:last-child)) {
        border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    }
    .album-song:hover {
        padding-right: 0px;
        cursor: pointer;
        padding-left: 25px;
        background-color: var(--deep-accent);
        color: var(--deep-accent-text);
    }

    .album-view-content-wrap {
        padding-bottom: 20px;
        display: inline-block;
        width: 100%;
        height: 100%;
        grid-template-columns: 200px 1fr;
        border-top: 1px solid var(--accent);
        border-bottom: 1px solid var(--accent);
        background-color: rgba(0, 39, 54, 0.9);
        backdrop-filter: blur(150px);
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
        color: var(--accent);
        font-weight: bold;
    }

    .active-song:hover {
        padding-right: 0px;
        background-color: var(--accent);
        color: var(--deep-accent);
        cursor: pointer;
        padding-left: 60px;
    }

    .album-name {
        height: 60px;
        width: calc(100vh - 100px);
        position: relative;
    }

    .album-metadata {
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        font-size: x-large;
        font-weight: bold;
        color: var(--text);
        display: flex;
    }

    .right {
        width: calc(100vw - 220px);
    }

    .album-img-wrap {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        z-index: 20;
        max-height: 200px;
    }

    .album-options-wrap {
        display: flex;
        justify-content: space-between;
        height: 100px;
    }

    .album-play-btn {
        aspect-ratio: 1/1;
        border: none;
        background-color: var(--secondary);
        color: var(--deep-accent-text);
        border-radius: 50px;
        padding: 8px;
        transition: 0.5s cubic-bezier(0.075, 0.82, 0.165, 1);
        margin-inline: 10px;
    }

    .album-play-btn:hover {
        background-color: var(--accent);
        cursor: pointer;
    }
</style>
