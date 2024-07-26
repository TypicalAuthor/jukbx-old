import { writable } from "svelte/store";

let library_size = writable<number>();

export { library_size }