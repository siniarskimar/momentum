<script>
    import { DateTime, Interval } from "luxon";

    /** @type {{viewDate?: DateTime }} */
    let { viewDate = $bindable(DateTime.now()) } = $props();

    const startOfWeek = viewDate.startOf("week");
    const endOfWeek = viewDate.endOf("week");
    const firstVisibleDay = viewDate.startOf("month").startOf("week");
    const lastVisibleDay = viewDate.endOf("month").endOf("week");

    const visibleDays = Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
        .splitBy({ days: 1 })
        .map((d) => d.start);

    const daysOfCurrWeek = Interval.fromDateTimes(startOfWeek, endOfWeek)
        .splitBy({ days: 1 })
        .map((d) => d.start);
</script>

<div class="container">
    <div class="topbar">
        {#each daysOfCurrWeek as day}
            <p class="weekday">
                {day
                    ?.toJSDate()
                    .toLocaleDateString(undefined, { weekday: "short" })}
            </p>
        {/each}
    </div>
    {#each visibleDays as day}
        <button class="day" class:exclusive={day?.month != viewDate.month}>
            <p class="day-number">{day?.day}</p>
        </button>
    {/each}
</div>

<style>
    .container {
        height: 100%;
        grid-column: 1 / -1;

        display: grid;
        grid-template-rows: auto repeat(6, minmax(5rem, 1fr));
        grid-template-columns: repeat(7, minmax(1rem, 1fr));
        grid-gap: 1px;
    }

    .topbar {
        grid-column: 1 / -1;

        position: sticky;
        top: 0;

        display: grid;
        grid-template-columns: subgrid;
        grid-template-rows: 1fr;
        grid-gap: 1px;

        z-index: 2;
        background-color: hsl(0, 0%, 100%);
    }

    @media (prefers-color-scheme: dark) {
        .topbar {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
    }

    .topbar > p {
        margin: 0;
        height: 100%;
        text-align: center;
        box-shadow:
            1px 0px 0px 0px rgba(0, 0, 0, 0.1),
            -1px 0px 0px 0px rgba(0, 0, 0, 0.1);
    }

    .topbar * {
        user-select: none;
    }

    .day {
        position: relative;

        outline: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 0;
    }

    .day-number {
        position: absolute;
        bottom: 0.01rem;
        left: 0.1rem;

        margin: 0;
        user-select: none;
        opacity: 0.6;
        transition: 150ms ease-in-out opacity;
    }

    .day:hover .day-number {
        opacity: 1;
    }

    .day.exclusive {
        background-color: rgba(80, 80, 80, 0.1);
    }

    .day.exclusive .day-number {
        opacity: 0.4;
    }
</style>
