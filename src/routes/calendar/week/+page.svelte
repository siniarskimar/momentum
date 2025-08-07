<script lang="ts">
  import { getViewDate } from "$lib/stores/calendar.svelte";
  import { DateTime } from "luxon";

  const viewDate = getViewDate();
</script>

<div class="week-view">
  {viewDate.date.toString()}
</div>

<style>
  .week-view {
    display: flex;
    justify-items: center;
    align-content: center;
  }
</style>
