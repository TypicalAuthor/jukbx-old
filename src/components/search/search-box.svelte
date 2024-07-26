<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import search from "../../assets/search.png";
    import type { SongMetadata } from "../../types/fs-types";
    import ResultBox from "./result-box.svelte";
    let search_value: string = "";
    let timer: number;
    let result: Array<SongMetadata> = [];
    let shouldShowResBox = false;

    $: if (!shouldShowResBox) {
        search_value = "";
    }

    function debounce() {
        clearTimeout(timer);
        result = [];
        if (search_value.trim() != "" && search_value != "") {
            timer = setTimeout(() => {
                invoke<Array<SongMetadata>>("search_string", {
                    searchFactor: search_value,
                }).then((val) => {
                    if (!shouldShowResBox) {
                        shouldShowResBox = true;
                    }
                    result = val;
                });
            }, 10);
        }
    }

    function showResBox() {
        shouldShowResBox = true;
    }

    function handleClickOut(
        e: MouseEvent & { currentTarget: EventTarget & HTMLElement } & {
            target: HTMLElement;
        },
    ) {
        if (e.target) {
            if (e.target.classList[0] != undefined) {
                if (
                    !e.target.classList[0].toString().startsWith("search-") &&
                    !e.target.classList[0].toString().startsWith("resbox-") &&
                    !e.target.classList[0].toString().startsWith("result-")
                ) {
                    shouldShowResBox = false;
                    search_value = "";
                }
            } else {
                shouldShowResBox = false;
                search_value = "";
            }
        }
    }
</script>

<svelte:body
    on:mousedown={(e) => {
        //@ts-ignore
        handleClickOut(e);
    }} />

<main class="search-wrapper">
    <!-- svelte-ignore a11y-missing-attribute -->
    <button class="search-filters-open"
        ><img src={search} width="10" height="10" /></button>
    <input
        class="search-box"
        type="text"
        placeholder="search"
        on:input={debounce}
        bind:value={search_value}
        on:click={showResBox} />

    {#if shouldShowResBox && result.length != 0 && search_value.trim().length != 0}
        <ResultBox {result} bind:shouldShowResBox></ResultBox>
    {/if}
</main>

<style>
    .search-wrapper {
        display: flex;
        z-index: 100;
    }
    .search-filters-open {
        border: none;
        background-color: var(--secondary);
        border: 1px solid var(--deep-accent);
        border-right: none;
        color: rgba(255, 255, 255, 0.5);
    }
    .search-box {
        width: 150px;
        height: 15px;
        background-color: var(--secondary);
        border: 1px solid var(--deep-accent);
        border-left: none;
        color: white;
    }
</style>
