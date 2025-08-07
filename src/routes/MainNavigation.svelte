<script lang="ts">
  import Icon from "@iconify/svelte";
  import { open as modalOpen } from "$lib/Modals.svelte";
  import AddCalendarObjectModal from "../lib/AddCalendarObjectModal.svelte";
  import { afterNavigate } from "$app/navigation";
  import { getActiveView } from "$lib/stores/calendar.svelte";

  const calendarView = getActiveView();

  let location = $state("/");

  afterNavigate((navigation) => {
    const to = navigation.to;
    if (!to) return;

    location = to.url.pathname;
  });
</script>

<div class="app-menu">
  <a
    href="/calendar/{calendarView.view}"
    class:active={location == "/calendar"}
  >
    <Icon icon="basil:calendar-solid" width="1.5em" height="1.5em" />
  </a>
  <a href="/tasks" class:active={location == "/tasks"}>
    <Icon icon="basil:checked-box-solid" width="1.5em" height="1.5em" />
  </a>
  <a href="/settings" class:active={location == "/settings"}>
    <Icon icon="basil:settings-alt-outline" width="1.5em" height="1.5em" />
  </a>
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

  a {
    box-shadow: none;
    height: 3em;
    min-width: 3em;
    padding: 0;

    display: flex;
    align-items: center;
    justify-content: center;
    color: black;
  }

  a.active {
    background-color: var(--color-bg1);
  }

  a:hover {
    background-color: var(--color-bg1);
  }

  .app-menu :global(.iconify),
  .app-menu :global(.icon) {
    width: 1.25rem;
    height: 1.25rem;
  }
</style>
