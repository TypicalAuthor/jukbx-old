<script lang="ts">
    import type { tableContent } from "../types/table-types";

    export let content: Array<tableContent>;
    export let tableHeight: string;
    export let tableWidth: string;
</script>

<main style="height:{tableHeight};">
    <table class="table-wrap" style="width: {tableWidth};">
        <thead>
            <tr>
                {#each content as header}
                    <th class="table-header">{header.header}</th>
                {/each}
            </tr>
        </thead>

        <tbody>
            {#each content as row}
                <td>
                    {#each row.content as data}
                        <tr>
                            {#if data.type == "text"}
                                {data.text}
                            {:else if data.type == "button"}
                                <button class="table-button">{data.text}</button>
                            {/if}
                        </tr>
                    {/each}
                </td>
            {/each}
        </tbody>
    </table>
</main>

<style>
    .table-wrap {
        border-collapse: collapse;
    }

    .table-header:not(:first-child) {
        border-left: 1px solid var(--accent);
    }

    .table-header {
        background-color: var(--primary);
        color: black;
        border-bottom: 1px solid var(--accent);
    }
    td:not(:first-child) {
        border-left: 1px solid var(--accent);
    }
    td {
        border-bottom: 1px solid var(--accent);
    }
    main {
        border: 2px solid var(--accent);
        background-color: var(--secondary);
        color: var(--text);
        overflow-y: auto;
    }

    .table-button {
        border: 1px solid var(--accent);
        margin: 5px;
        background-color: var(--background);
        color: var(--text);
        border-radius: 2px;
    }
</style>
