<script lang="ts">
  import MonthView from "./MonthView.svelte";
  import WeekView from "./WeekView.svelte";
  import { DateTime } from "luxon";
  import Icon from "@iconify/svelte";
  import type { CalendarView } from "$lib/ui";

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

  function navLeft() {
    switch (calendarView) {
      case "month":
        viewDate = viewDate.minus({ month: 1 });
        break;
      case "week":
        viewDate = viewDate.minus({ week: 1 });
        break;
      case "day":
        viewDate = viewDate.minus({ day: 1 });
        break;
    }
  }

  function navRight() {
    switch (calendarView) {
      case "month":
        viewDate = viewDate.plus({ month: 1 });
        break;
      case "week":
        viewDate = viewDate.plus({ week: 1 });
        break;
      case "day":
        viewDate = viewDate.plus({ day: 1 });
        break;
    }
  }

  function navToday() {
    viewDate = DateTime.now().startOf("day");
  }
</script>

<div class="container">
  <div class="action-bar">
    <h2 class="date-display">{viewDateString}</h2>
    <div class="navigation">
      <button class="nav-left" onclick={navLeft}>
        <Icon icon="basil:caret-left-solid" width="1.5em" height="1.5em" />
      </button>
      <button class="nav-now" onclick={navToday}>Today</button>
      <button class="nav-right" onclick={navRight}>
        <Icon icon="basil:caret-right-solid" width="1.5em" height="1.5em" />
      </button>
    </div>
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

  .action-bar {
    padding-left: 2em;
    display: flex;
    flex-direction: row;
  }

  .date-display {
    flex: 1;
    text-align: start;
    display: inline-block;
  }

  .navigation {
    display: flex;
    margin: 0.5em;
    padding: 0.5em;

    border-radius: 8px;
    border: 1px solid black;
  }

  .navigation button {
    background-color: transparent;
    box-shadow: none;
    border-radius: 0;
  }
</style>
