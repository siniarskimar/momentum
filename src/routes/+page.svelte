<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SettingsPage from "./SettingsPage.svelte";
  import { DateTime } from "luxon";
  import CalendarPage from "./CalendarPage.svelte";
  import Menu from "./Menu.svelte";
  import type { GlobalView, CalendarView } from "$lib/ui";

  let currentPage: GlobalView = $state(getLastSavedUiView());
  let calendarView: CalendarView = $state("month");

  $effect(() => {
    try {
      localStorage.setItem("last_ui_view", currentPage);
    } catch (err) {
      if (err instanceof DOMException) {
        console.warn(err.message); // not fatal
      }
    }
  });

  function getLastSavedUiView(): GlobalView {
    return (localStorage.getItem("last_ui_view") as GlobalView) || "calendar";
  }
</script>

<div class="container">
  <main>
    <Menu bind:currentView={currentPage} bind:calendarView />
    {#if currentPage === "calendar"}
      <CalendarPage bind:calendarView />
    {:else if currentPage === "settings"}
      <SettingsPage />
    {/if}
  </main>

  <div class="status-bar"></div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;

    margin: 0;
    text-align: center;
    height: 100vh;
  }

  main {
    display: flex;
    height: 100%;
    width: 100%;
  }

  .status-bar {
    height: 1.8em;
    padding-left: 0.5em;
    padding-right: 0.5em;
    display: flex;
    flex-direction: row;
    background-color: var(--color-bg1);
  }

  .status-bar button {
    all: unset;
    cursor: pointer;
  }

  .status-bar button:is(:hover, :focus) {
    outline: black auto 0.3rem;
  }
</style>
