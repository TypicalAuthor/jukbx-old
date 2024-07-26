import type { SongMetadata } from "./fs-types"

interface Queue {
    type: "search" | "album" | "library" | "playlist"
    contents: Array<SongMetadata>
    position_queue: number
    position_song_id: number
    album_name?: string
}

export type { Queue }