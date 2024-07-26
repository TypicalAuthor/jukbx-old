<script lang="ts">
    import type { menubar } from "../types/titlebar-types";
    import { appWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import TitlebarMenu from "./titlebar-menu.svelte";
    import minimizeSVG from "../assets/minimize-window-icon.svg";
    import maximizeSVG from "../assets/maximize-window-icon.svg";
    import closeSVG from "../assets/close-window-icon.svg";
    import SearchBox from "./search/search-box.svelte";

    export let windowName: string | null = null;
    export let menuBar: Array<menubar> | null = null;
    export let hasTextBoxSearch = false;

    let maximizable: boolean;
    let minimizable: boolean;
    let closable: boolean;

    let menuArr: Array<{ ID: number; isOpen: boolean; menuBar: menubar }> = [];

    if (menuBar) {
        for (let i = 0; menuBar.length > i; i++) {
            menuArr.push({ ID: i, isOpen: false, menuBar: menuBar[i] });
        }
    }
    let idPrevious: number;

    onMount(async () => {
        maximizable = await appWindow.isMaximizable();
        minimizable = await appWindow.isMinimizable();
        closable = await appWindow.isClosable();
    });
    function handleMouseEnter(e: CustomEvent<{ ID: number }>) {
        let hasOpened = !menuArr.every((e) => {
            return !e.isOpen;
        });
        if (hasOpened) {
            menuArr[idPrevious].isOpen = false;
            menuArr[e.detail.ID].isOpen = true;
            idPrevious = e.detail.ID;
            return;
        }
    }
    function handleMouseClick(e: CustomEvent<{ ID: number; showMenu: boolean }>) {
        menuArr[e.detail.ID].isOpen = !menuArr[e.detail.ID].isOpen;
        idPrevious = e.detail.ID;
    }

    function handleClickOut(
        e: MouseEvent & { currentTarget: EventTarget & HTMLElement } & {
            target: HTMLElement;
        },
    ) {
        if (e.target) {
            if (e.target.classList[0] != undefined) {
                if (
                    !e.target.classList[0].toString().startsWith("menu-") &&
                    !e.target.classList[0].toString().startsWith("menubar-")
                ) {
                    menuArr.forEach((elem) => {
                        elem.isOpen = false;
                        menuArr = menuArr;
                    });
                }
            } else {
                menuArr.forEach((elem) => {
                    elem.isOpen = false;
                    menuArr = menuArr;
                });
            }
        }
    }
</script>

<svelte:body
    on:mousedown={(e) => {
        //@ts-ignore
        handleClickOut(e);
    }} />

<main class="titlebar-margin">
    <div class="titlebar" data-tauri-drag-region>
        <div class="left">
            {#if windowName}
                <div class="window-name" data-tauri-drag-region>
                    {windowName}
                </div>
            {/if}
            {#if menuBar}
                <div class="menubar">
                    <div class="menubar-separator" data-tauri-drag-region></div>
                    {#each menuArr as menu (menu.ID)}
                        <TitlebarMenu
                            menu={menu.menuBar}
                            ID={menu.ID}
                            showMenu={menu.isOpen}
                            on:event={handleMouseEnter}
                            on:click={handleMouseClick}></TitlebarMenu>
                    {/each}
                </div>
            {/if}
        </div>
        <div class="right">
            {#if maximizable || minimizable || closable}
                {#if hasTextBoxSearch}
                    <SearchBox></SearchBox>
                {/if}
                <div class="window-controls">
                    {#if minimizable}
                        <button on:click={async () => await appWindow.minimize()}
                            ><img
                                src={minimizeSVG}
                                alt="minimize"
                                class="window-control-icon" /></button>
                    {/if}
                    {#if maximizable}
                        <button on:click={async () => await appWindow.toggleMaximize()}
                            ><img
                                src={maximizeSVG}
                                alt="maximize"
                                class="window-control-icon" /></button>
                    {/if}
                    {#if closable}
                        <button
                            on:click={async () => await appWindow.close()}
                            data-jukbx-window-close-btn
                            ><img
                                src={closeSVG}
                                alt="close"
                                class="window-control-icon" /></button>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</main>

<style>
    img {
        display: initial;
    }
    .titlebar-margin {
        background-color: var(--background);
        padding: 1px;
        border-bottom: 1px solid var(--accent);
    }
    .titlebar {
        padding: 5px var(--padding-inline) 5px var(--padding-inline);
        color: var(--text);
        background-color: var(--background);
        margin-top: 3px;
        margin-left: 3px;
        margin-right: 3px;
        padding-top: 0px;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .left {
        display: flex;
    }
    .right {
        display: flex;
        position: absolute;
        right: 24px;
        top: 0;
        align-items: center;
    }
    .menubar {
        display: flex;
        align-items: center;
    }
    .menubar-separator {
        border-left: 1px solid var(--text);
        margin-inline: 10px;
        height: 18px;
        opacity: 50%;
    }
    .window-name {
        font-style: italic;
        font-weight: bold;
        opacity: 50%;
    }
    .window-controls {
        display: flex;
        bottom: calc(100vh - 31px);
    }
    .window-controls > button {
        position: relative;
        right: -24px;
        height: 100%;
        border: none;
        background-color: rgba(255, 255, 255, 0);
        color: var(--text);
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
        width: 45px;
    }

    .window-controls > button:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
    .window-controls > button:hover[data-jukbx-window-close-btn] {
        background-color: var(--color-red);
    }

    .window-control-icon {
        width: 20px;
        height: 28px;
    }
</style>
