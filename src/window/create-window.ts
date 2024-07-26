import { WebviewWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri"

export { newLibraryManagerWindow, newAboutWindow, newSettingsWindow }

function newLibraryManagerWindow() {
    const webview = new WebviewWindow("jukbx-libraryManager-window", {
        url: "library_manager.html",
        decorations: false,
        skipTaskbar: true,
        alwaysOnTop: true,
        resizable: false,
        maximizable: false,
        minimizable: false,
        height: 350,
        width: 600
    })

    webview.once('tauri://created', function () {
        invoke('set_shadows', { label: webview.label })
    })

    webview.once('tauri://error', function (e) {
        console.log(`An error ocurred while creating a ${webview.label} window ${e}`)
    })
}

function newSettingsWindow() {
    const webview = new WebviewWindow("jukbx-settings-window", {
        url: "settings.html",
        decorations: false,
        skipTaskbar: true,
        alwaysOnTop: true,
        resizable: false,
        maximizable: false,
        minimizable: false,
        height: 350,
        width: 600
    })

    webview.once('tauri://created', function () {
        invoke('set_shadows', { label: webview.label })
    })

    webview.once('tauri://error', function (e) {
        console.log(`An error ocurred while creating a ${webview.label} window ${e}`)
    })
}

function newAboutWindow() {
    const webview = new WebviewWindow("jukbx-about-window", {
        url: "about.html",
        decorations: false,
        skipTaskbar: true,
        alwaysOnTop: true,
        resizable: false,
        maximizable: false,
        minimizable: false,
        height: 350,
        width: 600
    })

    webview.once('tauri://created', function () {
        invoke('set_shadows', { label: webview.label })
    })

    webview.once('tauri://error', function (e) {
        console.log(`An error ocurred while creating a ${webview.label} window ${e}`)
    })
}