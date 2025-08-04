<script lang="ts">
  import MonthView from "$lib/calendar/MonthView.svelte";
  import WeekView from "$lib/calendar/WeekView.svelte";
  import { DateTime, type DurationLike } from "luxon";
  import Icon from "@iconify/svelte";
  import type { CalendarView } from "$lib/ui";
  import Today from "$lib/icon/Today.svelte";

  interface Props {
    calendarView: CalendarView;
  }

  let { calendarView = $bindable("month") }: Props = $props();

  let viewDate: DateTime = $state(DateTime.now());
  let viewDateString: string | null = $state(null);

  function switchToWeek(day: DateTime) {
    viewDate = day.startOf("week");
    calendarView = "week";
  }

  function navigationAmount(view: CalendarView, amount: number): DurationLike {
    switch (view) {
      case "month":
        return { month: amount };
      case "week":
        return { week: amount };
      case "day":
        return { day: amount };
    }
  }

  function navLeft() {
    viewDate = viewDate.minus(navigationAmount(calendarView, 1));
  }

  function navRight() {
    viewDate = viewDate.plus(navigationAmount(calendarView, 1));
  }

  function navToday() {
    viewDate = DateTime.now().startOf("day");
  }
</script>

<div class="container">
  <div class="action-bar">
    <nav class="date-nav">
      <button class="nav-left" onclick={navLeft}>
        <Icon icon="basil:caret-left-solid" />
      </button>

      <button class="nav-now" onclick={navToday}>
        <Today day={viewDate.day} />
      </button>

      <button class="nav-right" onclick={navRight}>
        <Icon icon="basil:caret-right-solid" />
      </button>
    </nav>
    <h2 class="date-display">{viewDateString}</h2>
    <nav class="view-nav">
      <button
        class:active={calendarView === "month"}
        onclick={() => (calendarView = "month")}>M</button
      >
      <button
        class:active={calendarView === "week"}
        onclick={() => (calendarView = "week")}>W</button
      >
    </nav>
  </div>

  {#if calendarView == "month"}
    <MonthView
      bind:viewDate
      bind:viewDateString
      onDayLabelClicked={switchToWeek}
    />
  {:else if calendarView == "week"}
    <WeekView bind:viewDate bind:viewDateString />
  {/if}
</div>

<style>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  nav button {
    width: 2rem;
    height: 2rem;
    padding: 0;
    background-color: transparent;
    box-shadow: none;
  }

  nav button.active {
    background-color: dar;
  }

  .action-bar {
    padding-left: 1em;
    padding-right: 1em;
    display: flex;
    flex-direction: row;
  }

  .date-display {
    flex: 1;
    text-align: start;
    display: inline-block;
  }

  .date-nav {
    display: flex;
    margin-right: 1.2rem;
    align-items: center;
  }

  .date-nav button {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;

    font-size: 1.5rem;
  }

  .view-nav {
    display: flex;
    align-items: center;

    --border-radius: 4px;
    border-radius: var(--border-radius);
  }

  .view-nav button {
    box-sizing: content-box;
    border-radius: 0;
    padding: 0 0.2rem;
  }

  .view-nav button:nth-child(1) {
    border-top-left-radius: var(--border-radius);
    border-bottom-left-radius: var(--border-radius);
  }

  .view-nav button:last-child {
    border-top-right-radius: var(--border-radius);
    border-bottom-right-radius: var(--border-radius);
  }
</style>
