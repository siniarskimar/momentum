<script>
  import MonthView from "./MonthView.svelte";
  import WeekView from "./WeekView.svelte";
  import SettingsView from "./SettingsView.svelte";
  import { DateTime } from "luxon";

  /** @type {import('$lib/ui').GlobalView} */
  let currentView = $state("month");

  let viewDate = $state(DateTime.now());

  $effect(() => {
    try {
      localStorage.setItem("last_ui_view", currentView);
    } catch (err) {
      if (err instanceof DOMException) {
        console.warn(err.message); // not fatal
      }
    }
  });
</script>

<main class="container">
  {#if currentView === "month" || currentView === "week"}
    <div class="topbar">
      <p class="current-date">
        {
        viewDate.startOf('week')
          ?.toJSDate()
          .toLocaleDateString(undefined,
            currentView === "month" ? 
              { month: "long", year: "numeric" }
              : { day: "numeric" , month: "numeric", year: "numeric"}
          )
        }
      </p>
    </div>
  {/if}
  {#if currentView === "month"}
    <MonthView
      bind:viewDate
      onDayLabelClicked={(day) => {
        currentView = "week";
      }}
    />
  {:else if currentView === "week"}
    <WeekView
      bind:viewDate
      onMonthClicked={(start) => {
        currentView = "month";
      }}
    />
  {:else if currentView === "settings"}
    <SettingsView />
  {/if}

  <div class="action-bar">
    <div class="status-bar"></div>
    <div class="controls">
      <button>Add</button>
      <button>Settings</button>
    </div>
  </div>
</main>

<style>
  .container {
    display: flex;
    flex-direction: column;

    margin: 0;
    text-align: center;
    height: 100vh;
  }

  .action-bar {
    height: 1.8em;
    padding-left: 0.5em;
    padding-right: 0.5em;
    display: flex;
    flex-direction: row;
    background-color: var(--color-bg1);
  }

  .action-bar > * {
    flex: 1;
  }

  .controls {
    display: flex;
    justify-content: end;
  }

  .controls button {
    padding: 0.2em;
    margin: 0;
  }

  .topbar {
    display: flex;
    padding: 0 0.2em;
  }

  .current-date {
    text-align: left;
  }
</style>
