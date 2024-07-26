import { writable } from 'svelte/store';
export { playerStatus }
export type { Player }
import type { SongMetadata } from '../types/fs-types';

interface Player {
    paused: boolean,
    hasValidFile: boolean,
    currentTime: number,
    openFile: SongMetadata,
}

const playerStatus = writable<Player | null>(null);