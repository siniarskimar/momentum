<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SettingsView from "./SettingsView.svelte";
  import { DateTime } from "luxon";
  import CalendarView from "./CalendarView.svelte";
  import Menu from "./Menu.svelte";
  import type { GlobalView } from "$lib/ui";

  let currentView: GlobalView = $state(getLastSavedUiView());
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

  function getLastSavedUiView(): GlobalView {
    return (localStorage.getItem("last_ui_view") as GlobalView) || "calendar";
  }
</script>

<div class="container">
  <main>
    <Menu bind:currentView />
    {#if currentView === "calendar"}
      <CalendarView />
    {:else if currentView === "settings"}
      <SettingsView />
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

  .status-bar button:focus {
    outline: black auto 0.3rem;
  }
</style>
