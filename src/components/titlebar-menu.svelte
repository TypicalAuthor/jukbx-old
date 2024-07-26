<script lang="ts">
    import type { menubar } from "../types/titlebar-types";
    import { createEventDispatcher } from "svelte";
    import Menu from "./menu.svelte";

    export let menu: menubar;
    export let ID: number;
    export let showMenu: boolean;

    const dispatch = createEventDispatcher<{ event: { ID: number } }>();
    const dispatchCLICK = createEventDispatcher<{
        click: { ID: number; showMenu: boolean };
    }>();
    function mouseInEvent() {
        dispatch("event", {
            ID,
        });
    }

    function mouseClickEvent() {
        dispatchCLICK("click", {
            ID,
            showMenu,
        });
    }
</script>

<main>
    {#if showMenu}
        <Menu menuItems={menu.menu} on:click={mouseClickEvent}></Menu>
    {/if}
    <button class="menubar-button" on:click={mouseClickEvent} on:mouseenter={mouseInEvent}
        >{menu.title}</button>
</main>

<style>
    main {
        display: flex;
    }
    .menubar-button {
        border: none;
        background-color: transparent;
        color: var(--text);
        opacity: 80%;
        border-radius: 1px;
        height: 20px;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
    }
    .menubar-button:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
