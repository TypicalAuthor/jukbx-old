<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";

    export let sliderValue = 0;
    export let min = 0;
    export let max = 1;
    export let isPaused = true;

    let sliderEl!: HTMLDivElement;
    let sliderContainer!: HTMLDivElement;
    let sliderProgress!: HTMLDivElement;
    let boundingContainer: DOMRect;
    let boundingThumb: DOMRect;

    onMount(() => {
        boundingContainer = sliderContainer.getBoundingClientRect();
        boundingThumb = sliderEl.getBoundingClientRect();
        sliderEl.style.left = `${
            boundingContainer.left +
            convertToRelativeVal(
                min,
                max,
                sliderValue,
                boundingContainer.right,
                boundingContainer.left,
            ) -
            (boundingThumb.right - boundingThumb.left) / 2
        }px`;
        sliderProgress.style.width = `${convertToRelativeVal(
            min,
            max,
            sliderValue,
            boundingContainer.right,
            boundingContainer.left,
        )}px`;
    });

    export let onMouseDragEvent = false;

    $: if (!isPaused && !onMouseDragEvent) {
        boundingContainer = sliderContainer.getBoundingClientRect();
        boundingThumb = sliderEl.getBoundingClientRect();
        sliderEl.style.left = `${
            boundingContainer.left +
            convertToRelativeVal(
                min,
                max,
                sliderValue,
                boundingContainer.right,
                boundingContainer.left,
            ) -
            (boundingThumb.right - boundingThumb.left) / 2
        }px`;
        sliderProgress.style.width = `${convertToRelativeVal(
            min,
            max,
            sliderValue,
            boundingContainer.right,
            boundingContainer.left,
        )}px`;
    }

    let lastReturnVal = 0;

    function positionMouseMove(
        event: MouseEvent & { currentTarget: EventTarget & HTMLElement },
    ) {
        if (onMouseDragEvent) {
            event.preventDefault();
            let mouseXval = event.clientX;
            function returnVal() {
                if (mouseXval < boundingContainer.left) {
                    return (
                        boundingContainer.left -
                        (boundingThumb.right - boundingThumb.left) / 2
                    );
                }
                if (mouseXval > boundingContainer.right) {
                    return (
                        boundingContainer.right -
                        (boundingThumb.right - boundingThumb.left) / 2
                    );
                }
                return mouseXval - (boundingThumb.right - boundingThumb.left) / 2;
            }
            function returnValTrack() {
                if (mouseXval < boundingContainer.left) {
                    return boundingContainer.left;
                }
                if (mouseXval > boundingContainer.right) {
                    return boundingContainer.right;
                }
                return mouseXval;
            }
            boundingContainer = sliderContainer.getBoundingClientRect();
            boundingThumb = sliderEl.getBoundingClientRect();
            sliderEl.style.left = `${returnVal()}px`;
            sliderProgress.style.width = `${returnValTrack() - boundingContainer.left}px`;
            lastReturnVal = returnVal();
        }
    }
    const CHANGEEVENT = createEventDispatcher<{ update: { time: number } }>();

    function positionMouseUp(
        e: MouseEvent & { currentTarget: EventTarget & HTMLElement },
    ) {
        if (onMouseDragEvent) {
            CHANGEEVENT("update", {
                time: convertRelativePosToVal(
                    min,
                    max,
                    lastReturnVal,
                    boundingContainer.right,
                    boundingContainer.left,
                ),
            });

            onMouseDragEvent = false;
        }
    }

    function positionMouseDown(
        e: MouseEvent & { currentTarget: EventTarget & HTMLDivElement },
    ) {
        if (!onMouseDragEvent) {
            onMouseDragEvent = true;
            positionMouseMove(e);
        }
    }

    function convertToRelativeVal(
        min: number,
        max: number,
        value: number,
        right: number,
        left: number,
    ): number {
        //If the min is supperior than max then we dont need to do any calculations, its gonna be 0
        if (max < min) {
            return 0;
        }
        if (min > value) {
            return 0;
        }
        //Same thing, but if value surpases max
        if (value > max) {
            return right - left;
        }

        return (value / max - min) * (right - left);
    }

    function convertRelativePosToVal(
        min: number,
        max: number,
        positionX: number,
        right: number,
        left: number,
    ) {
        if (max < min) {
            return 0;
        }
        if (left > positionX) {
            return min;
        }

        return ((positionX - left) / (right - left)) * max;
    }

    function handleRezise() {
        boundingContainer = sliderContainer.getBoundingClientRect();
        boundingThumb = sliderEl.getBoundingClientRect();
        sliderEl.style.left = `${
            boundingContainer.left +
            convertToRelativeVal(
                min,
                max,
                sliderValue,
                boundingContainer.right,
                boundingContainer.left,
            ) -
            (boundingThumb.right - boundingThumb.left) / 2
        }px`;
        sliderProgress.style.width = `${convertToRelativeVal(
            min,
            max,
            sliderValue,
            boundingContainer.right,
            boundingContainer.left,
        )}px`;
    }
</script>

<svelte:body
    on:mousemove={(e) => {
        positionMouseMove(e);
    }}
    on:mouseup={(e) => {
        positionMouseUp(e);
    }} />
<svelte:window on:resize={handleRezise} />

<div class="slider-container" bind:this={sliderContainer}>
    <div
        class={`slider-thumb ${onMouseDragEvent ? "slider-thumb-visible" : ""}`}
        bind:this={sliderEl}
        role="slider"
        tabindex="0"
        aria-valuenow={sliderValue}
        on:mousedown={(e) => {
            positionMouseDown(e);
        }}>
    </div>
    <div
        class="slider-track"
        role="slider"
        tabindex="0"
        aria-valuenow={sliderValue}
        on:mousedown={(e) => {
            positionMouseDown(e);
        }}>
        <div class="slider-progress" bind:this={sliderProgress}></div>
    </div>
</div>

<style>
    .slider-track {
        background: var(--accent);
        height: 8px;
        border-radius: 16px;
        overflow: hidden;
        width: 100%;
        border: 1px solid var(--accent);
    }

    .slider-container {
        overflow: hidden;
        display: flex;
        align-items: center;
    }

    .slider-progress {
        background-color: var(--deep-accent);
        height: 100%;
        border-top-right-radius: 5px;
        border-bottom-right-radius: 5px;
    }

    .slider-thumb {
        background-color: var(--primary);
        width: 16px;
        aspect-ratio: 1/1;
        height: 16px;
        border-radius: 8px;
        position: absolute;
        opacity: 0;
    }

    .slider-thumb:hover {
        opacity: 1;
    }

    .slider-thumb-visible {
        opacity: 1;
        transition: none;
    }
</style>
