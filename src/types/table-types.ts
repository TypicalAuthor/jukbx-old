interface tableEntry {
    type: "button" | "text",
    text?: string
}

interface tableContent {
    header: string,
    headerID: number,
    content: Array<tableEntry>
}

export type { tableContent, tableEntry }