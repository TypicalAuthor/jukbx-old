<script lang="ts">
    import type { SongMetadata } from "../../types/fs-types";
    import type { Queue } from "../../types/queue-types";
    import { onDestroy } from "svelte";
    import { playerStatus } from "../../stores/playerStatus";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { activeSong } from "../../stores/song";
    import {
        audio_bufferLength,
        audio_dataArray,
    } from "../../stores/audio_data";
    import {
        isPlayerActive,
        isPlayerMini,
        showQueue,
    } from "../../stores/ui_state";
    import { currentQueue } from "../../stores/queue";
    import anime from "animejs";
    import Slider from "../slider.svelte";
    import Metadata from "./metadata.svelte";
    import Lyrics from "./lyrics.svelte";
    import Visualizer from "./visualizer.svelte";
    import pause from "../../assets/pause.png";
    import play from "../../assets/play.png";
    import foward_song from "../../assets/foward_song.png";
    import previous_song from "../../assets/previous_song.png";
    import queue_png from "../../assets/queue.png";
    import mini_toggle_down from "../../assets/mini_toggle_down.png";

    let player: HTMLAudioElement;
    let isPaused = true;
    let input_volume = 1;
    let currentFile: SongMetadata;
    let inputSliderMax = 1;
    let inputSliderValue = 0;
    let onMouseDragEvent: boolean;
    let local_analizer: AnalyserNode;
    let audioTotalTime: string = "00:00";
    let audioCurrentTime: string = "00:00";
    let ui_playerIsActive: boolean = false;
    let ui_isPlayerMini: boolean = false;
    let local_shouldShowQueue: boolean = false;
    let local_currentQueue: Queue;
    let currentSongIsLast = false;
    let currentSongIsFirst = false;

    const unsubPlayerActiveState = isPlayerActive.subscribe((state) => {
        ui_playerIsActive = state;
    });

    const unsubIsPlayerMini = isPlayerMini.subscribe((state) => {
        ui_isPlayerMini = state;
    });

    const uiShowQueueUnsub = showQueue.subscribe((state) => {
        local_shouldShowQueue = state;
    });

    const currentQueueUnsub = currentQueue.subscribe((queue) => {
        local_currentQueue = queue;
        if (local_currentQueue) {
            if (
                local_currentQueue.position_song_id ==
                local_currentQueue.contents[
                    local_currentQueue.contents.length - 1
                ].id
            ) {
                currentSongIsLast = true;
            } else {
                currentSongIsLast = false;
            }

            if (
                local_currentQueue.position_song_id ==
                local_currentQueue.contents[0].id
            ) {
                currentSongIsFirst = true;
            } else {
                currentSongIsFirst = false;
            }
        }
    });

    $: if (ui_playerIsActive && !ui_isPlayerMini) {
        anime.remove(".player_wrap");
        anime({
            targets: ".player_wrap",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ui_isPlayerMini ? "" : ["0%", "100%"],
            translateY: ["25px", "0px"],
            duration: 500,
        });
    }

    function formatTime(seconds: number) {
        if (seconds) {
            let res = `${Math.floor(seconds / 60)
                .toString()
                .padStart(2, "0")}:${Math.floor(seconds % 60)
                .toString()
                .padStart(2, "0")}`;
            if (res == undefined) {
                return "00:00";
            } else {
                return res;
            }
        } else {
            return "00:00";
        }
    }

    const UnSubOpenEvStore = activeSong.subscribe((file) => {
        if (file && player) {
            if (player.src) {
                inputSliderValue = 0;
                isPaused = true;
                player.pause();
                player.src = "";
            }
            player.src = convertFileSrc(file.location);
            console.log(file);
            if (player.src != undefined) {
                if (!ui_playerIsActive) {
                    isPlayerActive.set(true);
                }
                currentFile = file;
                inputSliderValue = 0;
                isPaused = true;
                local_analizer = createAudioContext(player);
                playerStatus.set({
                    paused: player.paused,
                    hasValidFile: true,
                    currentTime: player.currentTime,
                    openFile: file,
                });
            } else {
                playerStatus.set(null);
            }
        }
    });
    let dataArray = new Uint8Array();

    $: if (local_analizer && dataArray) {
        local_analizer.fftSize = 128;
        const bufferLength = local_analizer.frequencyBinCount;
        audio_bufferLength.set(local_analizer.frequencyBinCount);
        dataArray = new Uint8Array(bufferLength);
    }

    //This should really be an interval?
    let intID = setInterval(() => {
        if (local_analizer && dataArray && !isPaused) {
            local_analizer.getByteFrequencyData(dataArray);
            audio_dataArray.set(dataArray);
        }
    }, 1000 / 18);

    let audioSource: MediaElementAudioSourceNode | null = null;
    let analizer: AnalyserNode;

    function createAudioContext(src: HTMLMediaElement): AnalyserNode {
        src.crossOrigin = "anonymous";
        if (!audioSource) {
            const audioCtx = new window.AudioContext();
            audioSource = audioCtx.createMediaElementSource(src);
            analizer = audioCtx.createAnalyser();
            audioSource.connect(analizer);
            analizer.connect(audioCtx.destination);
        }
        return analizer;
    }

    function handleUpdate() {
        if (!onMouseDragEvent) {
            inputSliderValue = player.currentTime;
            audioCurrentTime = formatTime(player.currentTime);
            playerStatus.set({
                paused: player.paused,
                hasValidFile: true,
                currentTime: player.currentTime,
                openFile: currentFile,
            });
        }
    }

    //Handles the play() and pause() functions and updates playerStatus
    function handlePlay() {
        if (player.paused) {
            isPaused = false;
            player.play();
            playerStatus.set({
                paused: player.paused,
                hasValidFile: true,
                currentTime: player.currentTime,
                openFile: currentFile,
            });
        } else {
            isPaused = true;
            player.pause();
            playerStatus.set({
                paused: player.paused,
                hasValidFile: true,
                currentTime: player.currentTime,
                openFile: currentFile,
            });
        }
    }

    function handleMetadata() {
        handlePlay();
        inputSliderMax = player.duration;
        inputSliderValue = player.currentTime;
        audioTotalTime = formatTime(player.duration);
    }

    const PLAYBACK_FB = 5;

    function handleFoward() {
        player.currentTime = player.currentTime + PLAYBACK_FB;
    }

    function handleSliderUpdate(e: CustomEvent<{ time: number }>) {
        player.currentTime = e.detail.time;
        playerStatus.set({
            paused: player.paused,
            hasValidFile: true,
            currentTime: player.currentTime,
            openFile: currentFile,
        });
    }

    function handleBackwards() {
        player.currentTime = player.currentTime - PLAYBACK_FB;
    }

    function handleSmallAnim() {
        anime.remove(".player_wrap");
        anime({
            targets: ".player_wrap",
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            translateY: ["25px", "0px"],
            duration: 500,
        });
        isPlayerMini.set(true);
    }

    function shouldShowQueue() {
        showQueue.set(!local_shouldShowQueue);
    }

    function handleQueueFowards() {
        if (!currentSongIsLast && local_currentQueue) {
            activeSong.set(
                local_currentQueue.contents[
                    local_currentQueue.position_queue + 1
                ],
            );
            currentQueue.set({
                type: local_currentQueue.type,
                contents: local_currentQueue.contents,
                position_song_id: local_currentQueue.position_song_id + 1,
                position_queue: local_currentQueue.position_queue + 1,
                album_name: local_currentQueue.album_name,
            });
        }
    }

    function handleQueueBackWards() {
        if (!currentSongIsFirst && local_currentQueue) {
            activeSong.set(
                local_currentQueue.contents[
                    local_currentQueue.position_queue - 1
                ],
            );
            currentQueue.set({
                type: local_currentQueue.type,
                contents: local_currentQueue.contents,
                position_song_id: local_currentQueue.position_song_id - 1,
                position_queue: local_currentQueue.position_queue - 1,
                album_name: local_currentQueue.album_name,
            });
        } else {
            activeSong.set(
                local_currentQueue.contents[local_currentQueue.position_queue],
            );
        }
    }

    function handleEnded() {
        handleQueueFowards();
    }

    onDestroy(() => {
        uiShowQueueUnsub();
        UnSubOpenEvStore();
        unsubIsPlayerMini();
        currentQueueUnsub();
        unsubPlayerActiveState();
        if (intID) {
            clearInterval(intID);
        }
    });
</script>

<main class="player_wrap">
    <audio
        bind:this={player}
        bind:volume={input_volume}
        on:timeupdate={handleUpdate}
        on:loadedmetadata={handleMetadata}
        bind:paused={isPaused}
        on:ended={handleEnded}></audio>
    <div class="metadata">
        <div class="metadata_song">
            <Metadata></Metadata>
        </div>
        <div class="metadata-container">
            <div class="visualizer">
                <Visualizer></Visualizer>
            </div>
            <div class="lyrics">
                <Lyrics></Lyrics>
            </div>
        </div>
    </div>
    <div class="controls-container">
        <div class="controls">
            <div class="input-bar">
                <div class="time-tag_wrap">
                    <span class="time_tag">{audioCurrentTime}</span>
                    <span class="time_tag">{audioTotalTime}</span>
                </div>
                <div class="slider_wrap">
                    <Slider
                        min={0}
                        max={inputSliderMax}
                        sliderValue={inputSliderValue}
                        {isPaused}
                        on:update={handleSliderUpdate}
                        on:onMouseDragEvent></Slider>
                </div>
            </div>
            <div class="wrap">
                <div class="search-control">
                    <button class="" on:click={handleQueueBackWards}>
                        <!-- svelte-ignore a11y-missing-attribute -->
                        <img
                            src={previous_song}
                            height="20"
                            width="20" /></button>
                    <!-- svelte-ignore a11y-missing-attribute -->
                    <button on:click={handlePlay}
                        ><img
                            src={isPaused ? play : pause}
                            height="20"
                            width="20" /></button>
                    <button
                        class={currentSongIsLast ? "disabled" : ""}
                        disabled={currentSongIsLast}
                        on:click={handleQueueFowards}>
                        <!-- svelte-ignore a11y-missing-attribute -->
                        <img
                            src={foward_song}
                            height="20"
                            width="20" /></button>
                    <div class="line"></div>
                    <button on:click={handleFoward}>+{PLAYBACK_FB}s</button>
                    <button on:click={handleBackwards}>-{PLAYBACK_FB}s</button>
                    <div class="line"></div>
                    <!-- svelte-ignore a11y-missing-attribute -->
                    <button on:click={handleSmallAnim}
                        ><img
                            src={mini_toggle_down}
                            height="20"
                            width="20" /></button>
                    <!-- svelte-ignore a11y-missing-attribute -->
                    <button on:click={shouldShowQueue} class="queue-button">
                        <img
                            src={queue_png}
                            height="20"
                            width="20"
                            class="queue-img" /></button>
                </div>
                <div class="volume-control">
                    <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        bind:value={input_volume} />
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    .disabled {
        opacity: 0.5;
    }
    .time-tag_wrap {
        display: flex;
        justify-content: space-between;
        margin-top: 5px;
        margin-bottom: 5px;
    }
    .line {
        border-right: 1px solid rgba(255, 255, 255, 0.5);
        height: 20px;
        margin-right: 20px;
    }
    .search-control {
        display: flex;
        align-items: center;
    }
    .metadata-container {
        grid-column: 2;
        height: 80px;
        position: relative;
    }

    .time_tag {
        text-align: center;
        color: var(--text);
    }

    .input-bar {
        margin-bottom: 10px;
        align-items: center;
    }

    .slider_wrap {
        width: 100%;
    }
    .wrap {
        display: flex;
        width: calc(100vw - 40px);
        justify-content: space-between;
        align-items: center;
        text-align: center;
    }
    .metadata {
        display: grid;
        grid-template-columns: 400px 600px;
        gap: 50px;
        background-color: var(--deep-accent);
        border-top: 1px solid var(--accent);
        justify-content: space-between;
        align-items: center;
        border-bottom: 1px solid var(--accent);
    }

    .lyrics {
        position: relative;
        height: 100%;
        width: 100%;
        z-index: 10;
    }

    .visualizer {
        position: absolute;
        height: 100%;
        width: 100%;
        z-index: 1;
    }

    main {
        width: 100vw;
        box-shadow: 0px 0px 10px 1px rgba(0, 0, 0, 0.5);
        z-index: 10;
    }

    .controls-container {
        height: 80px;
    }

    .controls {
        padding-inline: 20px;
        width: calc(100vw - 40px);
        align-items: center;
        justify-items: center;
    }

    .volume-control {
        width: 140px;
    }

    button {
        background-color: transparent;
        color: white;
        border: none;
        padding-inline: 10px;
        margin-right: 20px;
        transition: all cubic-bezier(0.075, 0.82, 0.165, 1) 0.2s;
    }

    button:hover {
        background-color: #ffffff80;
        border-radius: 1px;
    }
</style>
