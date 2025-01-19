<script>
  import { DateTime } from "luxon";

  /**@type {{
        viewDate: import('luxon').DateTime,
        onMonthClicked: (start: import('luxon').DateTime) => void 
    }}*/
  let { viewDate = $bindable(DateTime.now()), onMonthClicked } = $props();
</script>

<div
  class="week-view"
  onclick={() => {
    onMonthClicked(viewDate);
  }}
>
  {viewDate.toString()}
</div>

<style>
  .week-view {
    display: flex;
    justify-items: center;
    align-content: center;
  }
</style>
