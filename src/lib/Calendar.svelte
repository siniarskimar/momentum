<script>
    import MonthView from "./calendar/MonthView.svelte";
    import { DateTime } from "luxon";

    /**
     * @typedef {'month' | 'week'} ViewEnum
     */

    /** @type {{view?: ViewEnum}}*/
    let { view = "month" } = $props();

    const currentDate = DateTime.now();
</script>

<div class="container">
    {#if view === "month"}
        <MonthView />
    {/if}
</div>

<style>
    .container {
        width: 100%;
        height: 100%;
    }
</style>
