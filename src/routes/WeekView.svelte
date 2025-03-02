<script lang="ts">
  import { DateTime } from "luxon";

  interface Props {
    viewDate: DateTime;
  }

  let { viewDate = $bindable(DateTime.now()) }: Props = $props();
</script>

<div class="week-view">
  {viewDate.toString()}
</div>

<style>
  .week-view {
    display: flex;
    justify-items: center;
    align-content: center;
  }
</style>
