<script>
    import { DateTime, Interval } from "luxon";

    const currDate = DateTime.now();
    const startOfWeek = currDate.startOf("week");
    const endOfWeek = currDate.endOf("week");
    const firstVisibleDay = currDate.startOf("month").startOf("week");
    const lastVisibleDay = currDate.endOf("month").endOf("week");

    const visibleDays = Interval.fromDateTimes(firstVisibleDay, lastVisibleDay)
        .splitBy({ days: 1 })
        .map((d) => d.start);

    const daysOfCurrWeek = Interval.fromDateTimes(startOfWeek, endOfWeek)
        .splitBy({ days: 1 })
        .map((d) => d.start);
</script>

<div class="container">
    <div class="topbar">
        <div class="currdate">
            {currDate.toJSDate().toLocaleDateString(undefined, {
                month: "long",
                year: "numeric",
            })}
        </div>
        {#each daysOfCurrWeek as day}
            <p class="weekday">
                {day
                    ?.toJSDate()
                    .toLocaleDateString(undefined, { weekday: "short" })}
            </p>
        {/each}
    </div>
    {#each visibleDays as day}
        <div class="day" class:exclusive={day?.month != currDate.month}>
            <p class="day-number">{day?.day}</p>
        </div>
    {/each}
</div>

<style>
    .container {
        height: 100%;

        display: grid;
        grid-template-rows: 4em repeat(6, minmax(5rem, 1fr));
        grid-template-columns: repeat(7, minmax(5rem, 1fr));
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

        box-shadow: 0px 2px 5px 1px rgba(0, 0, 0, 0.5);

        background-color: hsl(0, 0%, 100%);
        z-index: 2;
    }

    .topbar > p {
        text-align: center;
        box-shadow:
            1px 0px 0px 0px rgba(0, 0, 0, 0.1),
            -1px 0px 0px 0px rgba(0, 0, 0, 0.1);
    }

    .currdate {
        grid-column: 1 / -1;
        text-align: center;
    }

    .day {
        position: relative;

        outline: 1px solid rgba(0, 0, 0, 0.1);
    }

    .day-number {
        position: absolute;
        bottom: 0.01rem;
        left: 0.1rem;

        margin: 0;
    }

    .day.exclusive {
        background-color: rgba(80, 80, 80, 0.1);
    }

    .day.exclusive .day-number {
        color: rgba(40, 40, 40, 0.4);
    }
</style>
