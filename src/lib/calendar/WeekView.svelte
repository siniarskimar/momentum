<script lang="ts">
  import { DateTime } from "luxon";
  import { getLocaleString } from "$lib/api";

  interface Props {
    viewDate: DateTime;
    viewDateString: string | null;
  }

  const dateTimeFormat = new Intl.DateTimeFormat(getLocaleString(), {
    month: "long",
    year: "numeric",
    day: "numeric",
  });

  let {
    viewDate = $bindable(DateTime.now()),
    viewDateString = $bindable(null),
  }: Props = $props();

  const weekStart = $derived(viewDate.startOf("week"));
  const weekEnd = $derived(viewDate.endOf("week"));

  $effect(() => {
    viewDateString = dateTimeFormat.formatRange(
      weekStart.toJSDate(),
      weekEnd.toJSDate(),
    );
  });
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
