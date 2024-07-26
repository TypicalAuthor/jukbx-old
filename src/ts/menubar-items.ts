import type { menubar } from "../types/titlebar-types";
import { newAboutWindow, newLibraryManagerWindow, newSettingsWindow } from "../window/create-window"

let fileMenu: menubar = {
    title: "File",
    menu: [
        {
            type: "button",
            label: "Add files to library",
        },
        {
            type: "button",
            label: "Quick scan",
        },
        {
            type: "category",
            label: "Manage",
        },
        {
            type: "button",
            label: "Library manager",
            event: newLibraryManagerWindow,
        },
        {
            type: "button",
            label: "Playlist manager",
        },
        {
            type: "button",
            label: "Tag manager",
        },
    ],
};
let editMenu: menubar = {
    title: "Edit",
    menu: [
        {
            type: "button",
            label: "Settings",
            event: newSettingsWindow
        },
    ],
};

let helpMenu: menubar = {
    title: "Help",
    menu: [
        {
            type: "button",
            label: "About",
            event: newAboutWindow
        },
    ],
};

export { fileMenu, helpMenu, editMenu }