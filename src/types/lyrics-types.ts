interface lyrics {
    miliseconds: number;
    lyrics: string;
    active: boolean;
}

interface lrc {
    lyrics: Array<lyrics> | string | null;
}

export type { lyrics, lrc }