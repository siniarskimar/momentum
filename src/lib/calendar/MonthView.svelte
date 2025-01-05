<script>
    import { DateTime, Interval } from "luxon";

    /** @type {{viewDate?: DateTime }} */
    let { viewDate = $bindable(DateTime.now()) } = $props();

    const startOfWeek = viewDate.startOf("week");
    const endOfWeek = viewDate.endOf("week");
    const firstVisibleDay = viewDate.startOf("month").startOf("week");
    const lastVisibleDay = viewDate.endOf("month").endOf("week");

    const visibleDays = /** @type{DateTime[]} */ (Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
        .splitBy({ days: 1 })
        .map((d) => d.start));

    const visibleWeeks = /** @type{DateTime[]} */ (Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
        .splitBy({weeks: 1})
        .map((i) => i.start));

    const daysOfCurrWeek = Interval.fromDateTimes(startOfWeek, endOfWeek)
        .splitBy({ days: 1 })
        .map((d) => d.start);

    /**
     * @param {DateTime} start
     * @returns {DateTime[]}
     */
    function daysInWeek(start) {
        return /** @type {DateTime[]}*/ (Interval.fromDateTimes(start.startOf('week'), start.endOf('week'))
            .splitBy({days: 1})
            .map((i) => i.start));
    }
    
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
    {#each visibleWeeks as visibleWeek}
        <div class="day-row" role="row">
            {#each daysInWeek(visibleWeek) as day}
                <div class="day" class:exclusive={day?.month != viewDate.month}>
                    <button class="day-number">{day?.day}</button>
                </div>
            {/each}
        </div>
    {/each}
</div>

<style>
    .container {
        position: relative;
        height: 100%;
        grid-column: 1 / -1;
        
        display: flex;
        flex-direction: column;
    }

    .topbar {
        grid-column: 1 / -1;

        position: sticky;
        top: 0;

        display: flex;
        flex-direction: row;

        z-index: 2;
        background-color: hsl(0, 0%, 100%);
    }

    .topbar > * {
        user-select: none;
        flex: 1;
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

    .day-row {
        flex: 1;
        display: flex;
        flex-direction: row;
    }

    .day {
        flex: 1;
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
