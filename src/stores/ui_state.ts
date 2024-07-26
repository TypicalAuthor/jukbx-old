import { writable } from "svelte/store";

let isPlayerActive = writable(false);

let isPlayerMini = writable(false)

let libraryView = writable<"list" | "album">("album")

let showQueue = writable<boolean>(false)

let queueIsFixed = writable<boolean>(false)

export { isPlayerActive, isPlayerMini, libraryView, showQueue, queueIsFixed }