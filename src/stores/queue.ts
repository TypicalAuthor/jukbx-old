import { writable } from "svelte/store";
import type { Queue } from "../types/queue-types";

let currentQueue = writable<Queue>()

export { currentQueue }