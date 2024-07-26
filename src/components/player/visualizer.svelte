<script lang="ts">
    import { audio_bufferLength, audio_dataArray } from "../../stores/audio_data";
    import { onDestroy, onMount } from "svelte";

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null;

    onMount(() => {
        ctx = canvas.getContext("2d");
    });

    const width = 600;
    const height = 80;
    let bufferLength: number = 1;
    let x = 1;
    let barHeight: number;
    let barWidth: number;

    let buffLengthUnsub = audio_bufferLength.subscribe((length) => {
        if (length && canvas) {
            bufferLength = length;
            barWidth = canvas.width / bufferLength;
        }
    });
    let bufferData: Uint8Array;
    let debounce = 20;
    let dataArrayUnsubscribe = audio_dataArray.subscribe((data) => {
        if (data && bufferLength && ctx && canvas) {
            bufferData = data;
            callAnimate();
        }
    });

    let animID: number;
    let called = false;
    let lastTime = 0;
    let timer = 0;
    //18 FPS is the target
    let interval = 1000 / 18;

    function callAnimate() {
        if (!called) {
            animate(lastTime);
            called = true;
        }
    }

    function animate(timeStamp: number) {
        const deltaTime = timeStamp - lastTime;
        lastTime = timeStamp;
        if (timer > interval) {
            if (bufferData && ctx && canvas) {
                x = 1;
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                for (let i = 5; i < bufferLength; i++) {
                    barHeight = bufferData[i] / 4;
                    ctx.fillStyle = "rgb(5, 20, 25)";
                    ctx.fillRect(x, canvas.height - barHeight, barWidth, barHeight);
                    x += barWidth + 5;
                }
            }
            timer = 0;
        } else {
            timer += deltaTime;
        }

        animID = requestAnimationFrame(animate);
    }

    onDestroy(() => {
        dataArrayUnsubscribe();
        buffLengthUnsub();
        if (animID) {
            cancelAnimationFrame(animID);
        }
    });
</script>

<main>
    <canvas width="600px" height="80px" bind:this={canvas}></canvas>
</main>

<style>
    main {
        height: 100%;
        width: 100%;
    }

    canvas {
        width: 100%;
        height: 100%;
        opacity: 0.5;
    }
</style>
