<script lang="ts">
  import MonthView from "$lib/calendar/MonthView.svelte";
  import WeekView from "$lib/calendar/WeekView.svelte";
  import { DateTime } from "luxon";
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
        <Icon icon="basil:caret-left-solid" width="1em" height="1em" />
      </button>

      <button class="nav-right" onclick={navRight}>
        <Icon icon="basil:caret-right-solid" width="1em" height="1em" />
      </button>

      <button class="nav-now" onclick={navToday}>
        <Today day={viewDate.day} width="1em" height="1em" />
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

  .navigation {
    display: flex;
    align-items: center;
  }

  .navigation button {
    display: flex;
    align-items: center;
    justify-content: center;

    background-color: transparent;
    box-shadow: none;
    border-radius: 4px;

    height: 2rem;
    aspect-ratio: 1 / 1;

    padding: 0;

    font-size: 1.5rem;
  }
</style>
