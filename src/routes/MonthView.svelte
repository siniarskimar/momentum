<script lang="ts">
  import { DateTime, Interval } from "luxon";

  interface Props {
    viewDate: DateTime;
    onDayLabelClicked: (day: DateTime) => void;
  }

  let { viewDate = $bindable(DateTime.now()), onDayLabelClicked }: Props =
    $props();

  const startOfWeek = $derived(viewDate.startOf("week"));
  const endOfWeek = $derived(viewDate.endOf("week"));
  const firstVisibleDay = $derived(viewDate.startOf("month").startOf("week"));
  const lastVisibleDay = $derived(viewDate.endOf("month").endOf("week"));

  const visibleDays = $derived(
    Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
      .splitBy({ days: 1 })
      .map((d) => d.start) as DateTime[],
  );

  const visibleWeeks = $derived(
    Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
      .splitBy({ weeks: 1 })
      .map((i) => i.start) as DateTime[],
  );

  const daysOfCurrWeek = $derived(
    Interval.fromDateTimes(startOfWeek, endOfWeek)
      .splitBy({ days: 1 })
      .map((d) => d.start),
  );

  function daysInWeek(start: DateTime): DateTime[] {
    return Interval.fromDateTimes(start.startOf("week"), start.endOf("week"))
      .splitBy({ days: 1 })
      .map((i) => i.start) as DateTime[];
  }

  function isToday(datetime: DateTime): boolean {
    const today = DateTime.now();
    return (
      datetime.day === today.day &&
      datetime.month === today.month &&
      datetime.year == today.year
    );
  }
</script>

<div class="container">
  <div class="topbar">
    <div class="weekdays" role="row">
      {#each daysOfCurrWeek as day}
        <p class="weekday">
          {day?.toJSDate().toLocaleDateString(undefined, { weekday: "short" })}
        </p>
      {/each}
    </div>
  </div>
  {#each visibleWeeks as visibleWeek}
    <div class="day-row" role="row">
      {#each daysInWeek(visibleWeek) as daystamp}
        <div class="day" class:exclusive={daystamp.month != viewDate.month}>
          <button
            class="day-number-container"
            onclick={() => onDayLabelClicked(daystamp)}
          >
            <p class="day-number" class:day-today={isToday(daystamp)}>
              {daystamp.day}
            </p>
          </button>
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .container {
    position: relative;
    height: 100%;

    display: flex;
    flex-direction: column;
  }

  .topbar {
    position: sticky;
    top: 0;

    display: flex;
    flex-direction: column;

    z-index: 2;
    background-color: var(--color-bg);
  }

  .topbar * {
    user-select: none;
    flex: 1;
  }

  .weekdays {
    display: flex;
    flex-direction: row;
  }

  .weekday {
    flex: 1;

    margin: 0;
    height: 100%;
    text-align: center;
    box-shadow:
      1px 0px 0px 0px rgba(0, 0, 0, 0.1),
      -1px 0px 0px 0px rgba(0, 0, 0, 0.1);
  }

  .day-row {
    flex: 1;
    display: flex;
    flex-direction: row;
  }

  .day {
    flex: 1;
    position: relative;

    outline: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 0;
  }

  .day-number-container {
    position: absolute;
    bottom: 0.01rem;
    left: 0;
    right: 0;
    height: 2.5em;
    padding: 0.1em;

    border: none;
    box-shadow: none;
    background: transparent;
    margin: 0;
  }

  .day-number {
    width: 2em;
    height: 2em;
    display: flex;
    align-items: center;
    justify-content: center;

    border-radius: 50%;

    margin: 0;
    user-select: none;
    opacity: 0.6;
    transition: 150ms ease-in-out opacity;
  }

  .day-today {
    background-color: var(--color-bg1);
  }

  .day:hover .day-number {
    opacity: 1;
  }

  .day.exclusive {
    background-color: rgba(80, 80, 80, 0.1);
  }

  .day.exclusive .day-number {
    opacity: 0.4;
  }
</style>
