<script lang="ts">
    import type { SongMetadata } from "../../types/fs-types";
    import { onDestroy } from "svelte";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { activeSong } from "../../stores/song";
    import { isPlayerMini } from "../../stores/ui_state";
    import no_cover_alt from "../../assets/no_cover_alt.png";
    import mini_toggle from "../../assets/mini_toggle.png";
    import anime from "animejs";

    let open_metadata: SongMetadata | null;
    let anim: anime.AnimeInstance;

    const UnSubOpenEvStore = activeSong.subscribe((file) => {
        if (file) {
            anim = anime({
                targets: ".text_container",
                direction: "reverse",
                easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
                opacity: ["0%", "100%"],
                translateX: ["-25px", "0px"],
                duration: 250,
            });
            anim.finished.then(() => {
                open_metadata = file;
                anime({
                    targets: ".text_container",
                    direction: "normal",
                    easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
                    opacity: ["0%", "100%"],
                    translateX: ["-25px", "0px"],
                    duration: 250,
                }).finished.then(() => {
                    anime.remove(".text_container");
                });
            });
        }
    });

    let ui_isPlayerMini: boolean = false;

    const unsubIsPlayerMini = isPlayerMini.subscribe((state) => {
        ui_isPlayerMini = state;
    });

    function getSongImg(path: string | null) {
        if (path && path != "Some" && path != "None") {
            return convertFileSrc(path);
        }
        return no_cover_alt;
    }

    function handleMiniToggleUp() {
        isPlayerMini.set(false);
    }

    onDestroy(() => {
        unsubIsPlayerMini();
        UnSubOpenEvStore();
    });
</script>

<main>
    <div style="position: relative;">
        {#if ui_isPlayerMini}
            <!-- svelte-ignore a11y-missing-attribute -->
            <button class="mini-toggle" on:click={handleMiniToggleUp}
                ><img src={mini_toggle} width="65" height="80" /></button>
        {/if}
        <img
            width="80"
            height="80"
            alt="album cover"
            class="album_img"
            src={open_metadata?.image_path
                ? getSongImg(open_metadata?.image_path)
                : no_cover_alt} />
    </div>
    <div class="text_container">
        {#if open_metadata}
            {#if open_metadata.title}
                <div class="title">{open_metadata.title}</div>
            {:else}
                <div class="title">Unknown title</div>
            {/if}
            {#if open_metadata.artist}
                <div class="artist">{open_metadata.artist}</div>
            {:else}
                <div class="artist">Unknown artist</div>
            {/if}
        {:else}
            <div></div>
        {/if}
    </div>
</main>

<style>
    .mini-toggle {
        position: absolute;
        border: none;
        top: 0px;
        left: 0px;
        right: 0px;
        bottom: 0px;
        background-color: rgba(0, 39, 54, 0.5);
        color: white;
        opacity: 0;
        transition: 0.5s cubic-bezier(0.075, 0.82, 0.165, 1);
    }
    .mini-toggle:hover {
        opacity: 1;
    }
    main {
        display: flex;
        max-width: 100vw;
        align-items: center;
        gap: 10px;
        color: #fff;
    }
    .album_img {
        border: none;
        box-shadow: inset -2px -2px 10px 0px #00000080;
        z-index: 10;
    }

    .title {
        font-weight: bold;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        width: 350px;
    }

    .artist {
        font-weight: lighter;
        font-size: small;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        width: 350px;
    }
</style>
