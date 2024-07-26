/**
 * Type used for interactive elements on a ```menubar``` item
 */
interface menubarInput {
    icon?: HTMLImageElement
    label?: string
    event?: Function
    submenu?: menubar
}

/**
 * Type used for non-interactive elements on a ```menubar``` item
 */
interface menubarItem extends menubarInput {
    type: "category" | "submenu" | "button" | "range" | "separator"
}

/**
 * Type describing a basic ```menubar``` item
 */
interface menubar {
    title: string
    menu: Array<menubarItem>
}

/**
 * Type describing a basic button on a ```titlebar``` item
 */
interface menubarButton {
    title: string
    event: object
}

export type { menubar, menubarButton, menubarItem }