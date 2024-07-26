<script lang="ts">
    import { onDestroy } from "svelte";
    import { libraryView } from "../../stores/ui_state";
    let local_libraryView: "list" | "album";
    const unsubLibraryView = libraryView.subscribe((val) => {
        local_libraryView = val;
    });

    onDestroy(() => {
        unsubLibraryView();
    });
</script>

<main class="library-options">
    {#if local_libraryView}
        <button
            on:click={() => libraryView.set("album")}
            class={`${local_libraryView == "album" ? "active" : ""} library-option`}
            >Album covers</button>
        <div class="line"></div>
        <button
            on:click={() => libraryView.set("list")}
            class={`${local_libraryView == "list" ? "active" : ""} library-option`}
            >Albums and tracks</button>
    {/if}
</main>

<style>
    .library-options {
        height: 30px;
        background-color: var(--secondary);
        color: white;
        display: flex;
        align-items: center;
        padding-inline: 20px;
    }

    .active {
        background-color: var(--deep-accent) !important;
        color: var(--deep-accent-text) !important;
    }
    .library-option {
        background-color: var(--secondary);
        color: var(--text);
        border: none;
        opacity: 0.8;
        transition: 0.5s cubic-bezier(0.075, 0.82, 0.165, 1);
    }

    .line {
        height: 10px;
        border-right: 1px solid var(--accent);
        margin-inline: 10px;
    }

    .library-option:hover {
        background-color: var(--accent);
        color: var(--deep-accent);
        cursor: pointer;
    }
</style>
