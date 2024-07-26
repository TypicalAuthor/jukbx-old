import type { SongMetadata } from "./fs-types";

interface Album {
    name: string,
    artist?: string,
    year?: string,
    albumCoverPath?: string,
    contents: Array<SongMetadata>,
}

interface updateRes {
    songArray: Array<SongMetadata>;
}

export type { Album, updateRes }