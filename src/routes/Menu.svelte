<script lang="ts">
  import { slide } from "svelte/transition";
  import Icon from "@iconify/svelte";
  import type { GlobalView, CalendarView } from "$lib/ui";

  export interface Props {
    currentView: GlobalView;
    calendarView: CalendarView;
  }

  let {
    currentView = $bindable("calendar"),
    calendarView = $bindable("month"),
  }: Props = $props();
</script>

<div class="app-menu">
  <button
    class:active={currentView == "calendar"}
    onclick={() => (currentView = "calendar")}
  >
    <Icon icon="basil:calendar-solid" width="1.5em" height="1.5em" />
  </button>
  {#if currentView == "calendar"}
    <div class="view-actions" transition:slide>
      <button
        class:active={calendarView == "month"}
        onclick={() => (calendarView = "month")}
      >
        M
      </button>
      <button
        class:active={calendarView == "week"}
        onclick={() => (calendarView = "week")}
      >
        W
      </button>

      <button>
        <Icon icon="basil:plus-solid" width="1.5em" height="1.5em" />
      </button>
    </div>
  {/if}
  <button
    class:active={currentView == "settings"}
    onclick={() => (currentView = "settings")}
  >
    <Icon icon="basil:settings-alt-outline" width="1.5em" height="1.5em" />
  </button>
</div>

<style>
  .app-menu {
    width: 3.2em;
    padding: 0.2em;
    margin-right: 0.1em;

    display: flex;
    flex-direction: column;

    box-shadow: 1px 0 0.1em 1px var(--color-bg1);
  }

  .app-menu > * {
    margin-bottom: 0.1em;
  }

  button {
    box-shadow: none;
    height: 3em;
    min-width: 3em;
    padding: 0;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  button.active {
    background-color: var(--color-bg1);
  }

  button:hover {
    background-color: var(--color-bg1);
  }

  button.active:has(+ .view-actions) {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
    margin-bottom: 0;
  }

  .view-actions {
    display: flex;
    flex-direction: column;

    background-color: var(--color-bg1);
  }

  .view-actions button {
    background-color: transparent;
  }

  button.active + .view-actions {
    border-bottom-right-radius: 8px;
    border-bottom-left-radius: 8px;
  }

  .view-actions button.active {
    background-color: var(--color-bg2);
  }
</style>
