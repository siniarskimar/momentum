<script lang="ts">
  import { DateTime, type DurationLike } from "luxon";
  import Icon from "@iconify/svelte";
  import Today from "$lib/icon/Today.svelte";
  import { getActiveView, getViewDate } from "$lib/stores/calendar.svelte";

  let { children } = $props();

  let viewDate = getViewDate();
  let activeView = getActiveView();

  function navLeft() {
    viewDate.forwards();
  }

  function navRight() {
    viewDate.backwards();
  }

  function navToday() {
    viewDate.date = DateTime.now().startOf("day");
  }
</script>

<div class="container">
  <div class="action-bar">
    <nav class="date-nav">
      <button class="nav-left" onclick={navLeft}>
        <Icon icon="basil:caret-left-solid" />
      </button>

      <button class="nav-now" onclick={navToday}>
        <Today day={viewDate.date.day} />
      </button>

      <button class="nav-right" onclick={navRight}>
        <Icon icon="basil:caret-right-solid" />
      </button>
    </nav>
    <h2 class="date-display">{viewDate.format}</h2>
    <nav class="view-nav">
      <a href="./month" class:active={activeView.view === "month"}>
        <Icon icon="fa:table" />
      </a>
      <a href="./week" class:active={activeView.view === "week"}>
        <Icon icon="bx:columns" />
      </a>
    </nav>
  </div>
  <div class="calendar-view">
    {@render children()}
  </div>
</div>

<style>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .calendar-view {
    height: 100%;
  }

  button {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  nav button {
    width: 2rem;
    height: 2rem;
    padding: 0;
    background-color: transparent;
    box-shadow: none;
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
  }

  .view-nav {
    display: flex;
    align-items: center;

    --border-radius: 4px;
    border-radius: var(--border-radius);
  }

  .view-nav a {
    box-sizing: content-box;
    border-radius: 0;
    padding: 0 0.2rem;
  }

  .view-nav a:nth-child(1) {
    border-top-left-radius: var(--border-radius);
    border-bottom-left-radius: var(--border-radius);
  }

  .view-nav a:last-child {
    border-top-right-radius: var(--border-radius);
    border-bottom-right-radius: var(--border-radius);
  }
</style>
