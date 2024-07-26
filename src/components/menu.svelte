<script lang="ts">
    import type { menubarItem } from "../types/titlebar-types";
    import { onMount } from "svelte";
    import anime from "animejs";

    export let menuItems: Array<menubarItem>;
    export let isSubmenu: boolean = false;

    let subMenuShow: boolean = false;
    onMount(() => {
        anime({
            targets: "#menuItem",
            paddingLeft: ["0px", "25px"],
            direction: "normal",
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            opacity: ["0%", "100%"],
            duration: 500,
            delay: function (_el, i) {
                return i * 15;
            },
        });
        document.querySelectorAll("#menuItem").forEach((e) => {
            e.id = "called-menuItem";
        });
    });

    function hoverAnim(
        el: EventTarget,
        element: HTMLElement,
        padding: number,
        color: string,
        backgroundColor: string,
        filter: number,
    ) {
        anime.remove(el);
        anime({
            targets: el,
            paddingLeft: padding,
            direction: "normal",
            color: color,
            backgroundColor: backgroundColor,
            update: function (anim) {
                element.style.filter = "saturate(" + anim.progress * filter + "%)";
            },
            easing: "cubicBezier(0.075, 0.82, 0.165, 1)",
            duration: 500,
        });
    }

    function AnimMouseEnter(
        e:
            | (MouseEvent & { currentTarget: EventTarget & HTMLButtonElement })
            | (MouseEvent & { currentTarget: EventTarget & HTMLDivElement }),
    ) {
        hoverAnim(
            e.target!,
            e.currentTarget,
            35,
            "rgb(235, 242, 244)",
            "rgb(18, 30, 33)",
            5,
        );
    }
    function AnimMouseLeave(
        e:
            | (MouseEvent & { currentTarget: EventTarget & HTMLButtonElement })
            | (MouseEvent & { currentTarget: EventTarget & HTMLDivElement }),
    ) {
        hoverAnim(
            e.target!,
            e.currentTarget,
            25,
            "rgb(0, 0, 0)",
            "rgb(183, 205, 210)",
            1,
        );
    }
</script>

<main class={isSubmenu ? "menu-submenu-wrapper" : "menu-wrapper"}>
    <div class="menu-position">
        {#each menuItems as item}
            {#if item.type == "button" && item.label != undefined}
                <button
                    class="menu-button"
                    on:click={() => {
                        if (item.event != undefined) {
                            item.event();
                        }
                    }}
                    on:mouseenter={(e) => AnimMouseEnter(e)}
                    on:mouseleave={(e) => AnimMouseLeave(e)}
                    on:mouseenter={() => (subMenuShow = false)}
                    on:click
                    id="menuItem">{item.label}</button>
            {/if}
            {#if item.type == "category" && item.label != undefined}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <div class="menu-category" on:mouseenter={() => (subMenuShow = false)}>
                    {item.label}
                </div>
            {/if}
            {#if item.type == "separator"}
                <div class="menu-separator"></div>
            {/if}
            {#if item.type == "range" && item.label != undefined}
                <div class="menu-range"></div>
            {/if}
            {#if item.type == "submenu" && item.submenu != undefined}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <div
                    class="menu-submenu"
                    id="menuItem"
                    on:mouseenter={(e) => AnimMouseEnter(e)}
                    on:mouseleave={(e) => AnimMouseLeave(e)}
                    on:mouseenter={() => (subMenuShow = true)}>
                    {item.submenu.title}
                </div>
                {#if subMenuShow}
                    <svelte:self menuItems={item.submenu.menu} isSubmenu={true}
                    ></svelte:self>
                {/if}
            {/if}
        {/each}
    </div>
</main>

<style>
    .menu-wrapper {
        position: relative;
        top: 20px;
    }
    .menu-submenu-wrapper {
        position: relative;
        left: 150px;
        top: -22px;
    }
    .menu-position {
        position: absolute;
        border: 2px solid black;
        color: black;
        background-color: var(--primary);
        width: 150px;
        box-shadow: 0px 10px 15px 1px rgba(0, 0, 0, 0.3);
        padding: 2px;
        z-index: 1000;
    }
    .menu-category {
        color: var(--text);
        background-color: var(--secondary);
        font-size: smaller;
        padding: 0px 10px 3px 10px;
        font-style: italic;
        text-align: left;
    }
    .menu-button {
        display: flex;
        width: 100%;
        height: 20px;
        border: none;
        padding-left: 25px;
        background-color: var(--primary);
        color: black;
        text-align: left;
    }

    .menu-submenu {
        display: flex;
        height: 20px;
        border: none;
        background-color: var(--primary);
        color: black;
        text-align: left;
        font-size: smaller;
    }
    .menu-separator {
        border-bottom: 1px solid rgba(0, 0, 0, 0.3);
    }
</style>
