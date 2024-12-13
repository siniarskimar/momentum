<script>
  // import { invoke } from "@tauri-apps/api/core";
  import Calendar from "$lib/Calendar.svelte";

  /**@type {import('$lib/types').CalendarView}*/
  let calendarView = $state("month");
</script>

<main class="container">
  <Calendar bind:view={calendarView} />
</main>

<style>
  .container {
    display: grid;
    margin: 0;
    text-align: center;
    height: 100vh;
  }
</style>
