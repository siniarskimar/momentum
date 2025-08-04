<script lang="ts">
  import "../global.css";
  import MainNavigation from "./MainNavigation.svelte";
  import Modals from "$lib/Modals.svelte";

  let { children } = $props();
</script>

<div class="container">
  <main>
    <MainNavigation />
    {@render children()}
  </main>

  <div class="status-bar"></div>
  <Modals />
</div>

<style>
  .container {
    position: relative;
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
    height: 1em;
    padding-left: 0.5em;
    padding-right: 0.5em;
    display: flex;
    flex-direction: row;
    background-color: var(--color-bg1);
  }
</style>
