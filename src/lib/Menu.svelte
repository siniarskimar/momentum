<script lang="ts">
  import Icon from "@iconify/svelte";
  import type { GlobalView, CalendarView } from "$lib/ui";
  import { open as modalOpen } from "$lib/Modals.svelte";
  import AddCalendarObjectModal from "./AddCalendarObjectModal.svelte";

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
  <button
    class:active={currentView == "tasks"}
    onclick={() => (currentView = "tasks")}
  >
    <Icon icon="basil:checked-box-solid" width="1.5em" height="1.5em" />
  </button>
  <button
    class:active={currentView == "settings"}
    onclick={() => (currentView = "settings")}
  >
    <Icon icon="basil:settings-alt-outline" width="1.5em" height="1.5em" />
  </button>
  <div class="global-actions">
    <button
      onclick={() => {
        modalOpen(AddCalendarObjectModal, {});
      }}
    >
      <Icon icon="basil:plus-solid" width="1.5em" height="1.5em" />
    </button>
  </div>
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

  .global-actions {
    flex: 1;
    display: flex;
    align-items: end;
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

  .app-menu :global(.iconify), .app-menu :global(.icon) {
    width: 1.25rem;
    height: 1.25rem;
  }
</style>
