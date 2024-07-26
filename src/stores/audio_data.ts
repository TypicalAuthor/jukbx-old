import { writable } from "svelte/store";

let audio_dataArray = writable<Uint8Array>(undefined);
let audio_bufferLength = writable<number>(undefined)

export { audio_dataArray, audio_bufferLength }