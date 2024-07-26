<script lang="ts">
    import type { lyrics, lrc } from "../../types/lyrics-types";
    import { onDestroy } from "svelte";
    import { playerStatus } from "../../stores/playerStatus";
    import { invoke } from "@tauri-apps/api/tauri";
    import { activeSong } from "../../stores/song";

    let open_lyrics: Array<lyrics> | string | null;
    let timer = 0;
    let localtimerID: number;
    let alreadyWithLocalT: boolean = false;

    //Lyrics arent synced perfectly, so we add a bit of delay to the calculations, this should be configurable from settings
    const NATURAL_DELAY = 300;

    //Request the lyrics metadata tag from the backend, if something fails of there is none, return null
    const UnSubOpenEvStore = activeSong.subscribe(async (file) => {
        if (file) {
            await invoke<lrc>("request_lyrics", {
                fileLocation: file.location,
            })
                .then((res) => {
                    if (res) {
                        open_lyrics = res.lyrics;
                    } else {
                        open_lyrics = null;
                    }
                })
                .catch(() => {
                    open_lyrics = null;
                });
        }
    });

    //If player and open_lyrics are defined, then create a timer that grabs the player's currentTime
    //every time it fires an 'timeUpdate' event, aside from that create a local timer that updates every 10ms
    //and assign it while we wait for the 'timeUpdate' event in the player
    //Destroy the time is there is one and the player's current status is 'paused'
    const UnSubCurrentTime = playerStatus.subscribe((player) => {
        if (player && open_lyrics) {
            timer = player.currentTime * 1000;
            if (player.hasValidFile && !player.paused && !alreadyWithLocalT) {
                localtimerID = setInterval(() => {
                    timer = timer + 10;
                }, 10);
                alreadyWithLocalT = true;
            }
            if (player.paused && localtimerID) {
                clearInterval(localtimerID);
            }
        }
    });

    let last_lyrics = "";

    $: if (open_lyrics) {
        if (Array.isArray(open_lyrics)) {
            open_lyrics.forEach((lyrics) => {
                lyrics.active = lyrics.miliseconds <= timer + NATURAL_DELAY;
            });
            //? If there is lyrics that are marked as 'active', then select the last one, if there is none, return and empty string
            last_lyrics = open_lyrics[
                open_lyrics.filter((l) => l.active == true).length - 1
            ]
                ? open_lyrics[
                      open_lyrics.filter((l) => l.active == true).length - 1
                  ].lyrics
                : "";
        }
    }

    onDestroy(() => {
        UnSubOpenEvStore();
        UnSubCurrentTime();
        if (localtimerID) {
            clearInterval(localtimerID);
        }
    });
</script>

<main>
    {#if open_lyrics}
        {#if Array.isArray(open_lyrics)}
            <div class="bold_lyrics">{last_lyrics}</div>
        {:else}
            <div>{open_lyrics}</div>
        {/if}
    {:else}
        <div class="not_found">No lyrics found</div>
    {/if}
</main>

<style>
    ::-webkit-scrollbar {
        width: 0px;
        height: 10px;
    }

    ::-webkit-scrollbar-track {
        background: transparent;
    }

    ::-webkit-scrollbar-thumb {
        background: transparent;
    }

    ::-webkit-scrollbar-thumb:hover {
        background: transparent;
    }

    main {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: calc(100% - 40px);
        max-height: 80px;
        overflow-y: auto;
        overflow-x: hidden;
        color: white;
    }

    .not_found {
        opacity: 0.5;
    }

    .bold_lyrics {
        font-weight: bold;
        font-size: larger;
        margin: 0;
    }

    div {
        white-space: pre-wrap;
        text-align: center;
    }
</style>
