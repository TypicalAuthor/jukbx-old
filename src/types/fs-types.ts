interface File {
    album: null | string,
    artist: null | string,
    location: string,
    id: number,
    track_number: null | string,
    image_path: null | string
    title: null | string,
    year: null | number,
}

interface Folder {
    id: number,
    folder_path: string,
    file_amount: number | null,
}

export type { File as SongMetadata, Folder }