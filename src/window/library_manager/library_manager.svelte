<script lang="ts">
    import type { tableContent, tableEntry } from "../../types/table-types";
    import Titlebar from "../../components/titlebar.svelte";
    import Table from "../../components/table.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import type { SongMetadata, Folder } from "../../types/fs-types";
    import { onDestroy, onMount } from "svelte";
    import type { library_payload } from "../../types/event-types";
    import { library_size } from "../../stores/library";
    let song_amount = 0;
    let libraryTable: tableContent = {
        header: "Folders",
        headerID: 0,
        content: [],
    };

    onMount(async () => {
        await invoke<Array<SongMetadata>>("request_all_libraries").then(async (res) => {
            song_amount = res.length;
            library_size.set(res.length);
        });
        await invoke<Array<Folder>>("request_folders").then(async (res) => {
            let res_array: Array<tableEntry> = [];
            for (let i = 0; i < res.length; i++) {
                res_array.push({
                    type: "text",
                    text: res[i].folder_path,
                });
            }
            libraryTable.content = res_array;
        });
        generateListener();
    });

    let show_preview = false;
    let preview: string | undefined;
    let unlistenProccessing: UnlistenFn;
    let unlistenFinished: UnlistenFn;

    async function generateListener() {
        unlistenProccessing = await listen<library_payload>(
            "processing_file",
            (event) => {
                if (event) {
                    preview = event.payload.path;
                    if (!show_preview) {
                        show_preview = true;
                    }
                }
            },
        );

        unlistenFinished = await listen("proccessing_finished", () => {
            show_preview = false;
            preview = undefined;
        });
    }

    async function userInsert(folder: string) {
        isLoading = true;
        await invoke("register_folder", { folderLocation: folder }).then(async () => {
            isLoading = false;
            await invoke<Array<SongMetadata>>("request_all_libraries").then(
                async (res) => {
                    await emit("library_updated", { songArray: res });
                    song_amount = res.length;
                    library_size.set(res.length);
                },
            );
            await invoke<Array<Folder>>("request_folders").then(async (res) => {
                let res_array: Array<tableEntry> = [];
                for (let i = 0; i < res.length; i++) {
                    res_array.push({
                        type: "text",
                        text: res[i].folder_path,
                    });
                }
                libraryTable.content = res_array;
            });
        });
    }

    let isLoading = false;
    async function register_folder() {
        const folder = await open({
            directory: true,
            multiple: false,
        });
        if (Array.isArray(folder)) {
            console.log("Should not be array!");
        } else if (folder != null) {
            userInsert(folder);
        }
    }

    onDestroy(() => {
        unlistenProccessing();
        unlistenFinished();
    });
</script>

<main>
    <Titlebar windowName="Library Manager"></Titlebar>
    <div class="component-wrap">
        <Table content={[libraryTable]} tableWidth="100%" tableHeight="200px"></Table>
    </div>
    <div class="options">
        <button on:click={register_folder}>Add folder</button>
    </div>

    {#if isLoading}
        <div class="loadingProgress">Loading...</div>
        {#if show_preview}
            {#if preview}
                <div class="loading_preview">{preview}</div>
            {/if}
        {/if}
    {/if}

    <div class="bottom">
        <div>Songs found: {song_amount}</div>
    </div>
</main>

<style>
    .component-wrap {
        margin-top: 20px;
        padding-inline: var(--padding-inline);
    }

    .bottom {
        position: absolute;
        bottom: 10px;
        color: var(--text);
        left: 0;
        right: 0;
    }

    .bottom > div {
        font-size: small;
        opacity: 50%;
        text-align: right;
        padding-inline: var(--padding-inline);
    }

    .options {
        margin-top: 10px;
        padding-inline: var(--padding-inline);
    }

    .loadingProgress {
        padding-inline: var(--padding-inline);
        color: var(--text);
        margin-top: 5px;
    }

    .loading_preview {
        padding-inline: var(--padding-inline);
        color: var(--text);
        margin-top: 5px;
        opacity: 0.5;
        font-size: small;
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
        width: 450px;
    }

    button {
        border: 1px solid transparent;
        background-color: var(--primary);
        border-radius: 2px;
        width: 90px;
        transition: 500ms cubic-bezier(0.075, 0.82, 0.165, 1);
    }

    button:hover {
        border: 1px solid rgb(170, 180, 180);
        background-color: var(--secondary);
        color: var(--text);
        filter: saturate(500%);
        cursor: pointer;
    }
</style>
