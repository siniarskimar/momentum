<script>
    import MonthView from "./calendar/MonthView.svelte";
    import { DateTime } from "luxon";

    /** @type {{
     *      view?: import('$lib/ui').CalendarView,
     *      date?: import('luxon').DateTime
     *  }}
     */
    let { view = $bindable("month"), date = $bindable(DateTime.now()) } =
        $props();
</script>

<div class="container">
    <div class="topbar">
        <div class="viewselect">
            <button
                class:view-selected={view == "month"}
                onclick={() => {
                    view = "month";
                }}>M</button
            >
            <button
                class:view-selected={view == "week"}
                onclick={() => {
                    view = "week";
                }}>W</button
            >
        </div>
        <p class="currdate">
            {date.toJSDate().toLocaleDateString(undefined, {
                month: "long",
                year: "numeric",
            })}
        </p>
        <div class="spacer"></div>
    </div>
    {#if view === "month"}
        <MonthView bind:viewDate={date} />
    {:else if view == "week"}
        <p>Week view</p>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .topbar {
        display: flex;
    }

    .topbar > * {
        flex: 1;
    }

    @media (prefers-color-scheme: dark) {
        .topbar {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
    }

    .viewselect {
        display: flex;
        flex-direction: row;
        justify-content: start;
        align-items: center;
    }

    .viewselect button {
        border-radius: 0;
        border-top-left-radius: 0.2em;
        border-top-right-radius: 0.2em;
        border: 1px solid rgba(0, 0, 0, 0.2);

        margin-left: 0.3rem;
        margin-right: 0.3rem;
        padding-top: 0.5rem;
        padding-bottom: 0;
    }

    .viewselect .view-selected {
        background-color: chocolate;
    }

    .currdate {
        text-align: center;

        margin: 0;
    }
</style>
