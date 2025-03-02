<script lang="ts">
  import MonthView from "./MonthView.svelte";
  import WeekView from "./WeekView.svelte";
  import { DateTime } from "luxon";
  import { getLocaleString } from "$lib/api";

  let currentView: "month" | "week" = $state("month");
  let viewDate: DateTime = $state(DateTime.now());

  function switchToWeek(day: DateTime) {
    viewDate = day.startOf("week");
    currentView = "week";
  }

  const dateTimeFormat = new Intl.DateTimeFormat(getLocaleString(), {});
  const dateParts = $derived(dateTimeFormat.formatToParts(viewDate.toJSDate()));
  const datePartsFiltered = $derived(
    dateParts.filter((v) => {
      if (
        v.type === "day" &&
        (currentView === "month" || currentView === "week")
      ) {
        return false;
      }
      return true;
    }),
  );
</script>

<div class="container">
  <div class="action-bar">
    <p class="date-display">
      {#each datePartsFiltered as part}
        {#if part.type !== "literal"}
          <button class="date-part">
            {part.value}
          </button>
        {:else}
          <span class="date-part" class:date-literal={part.type === "literal"}>
            {part.value}
          </span>
        {/if}
      {/each}
    </p>
    <div class="navigation">
      <button>Prev</button>
      <button>Next</button>
    </div>
  </div>

  {#if currentView == "month"}
    <MonthView bind:viewDate onDayLabelClicked={switchToWeek} />
  {:else if currentView == "week"}
    <WeekView bind:viewDate />
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
    display: flex;
    flex-direction: row;
  }
  .date-display {
    display: inline-block;
  }
</style>
