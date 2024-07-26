import { writable } from "svelte/store";
import type { SongMetadata } from "../types/fs-types";
import { isPlayerActive } from "./ui_state";

let activeSong = writable<SongMetadata>()

function setActiveSong(i: SongMetadata) {
    if (i) {
        activeSong.set(i);
        isPlayerActive.set(true);
    }
}


export { activeSong, setActiveSong }