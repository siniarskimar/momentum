import { goto } from "$app/navigation";
import { getDateTimeFormatOptions, getLocaleString, type CalendarView } from "$lib/calendar";
import { DateTime, type DurationLike } from "luxon";

let selectedDate = $state(DateTime.now());
let activeView: CalendarView = $state('month');
const dateTimeFormatter = $derived(new Intl.DateTimeFormat(getLocaleString(), getDateTimeFormatOptions(activeView)));

function navigationAmount(view: CalendarView, amount: number): DurationLike {
    switch (view) {
        case "month":
            return { month: amount };
        case "week":
            return { week: amount };
        case "day":
            return { day: amount };
    }
}

function formatDateTime(date: DateTime, view: CalendarView): string {
    if (view === 'week') {
        return dateTimeFormatter.formatRange(date.startOf('week').toJSDate(), date.endOf('week').toJSDate());
    }

    return dateTimeFormatter.format(date.toJSDate());
}


export function getViewDate() {
    function forwards() {
        selectedDate = selectedDate.plus(navigationAmount(activeView, 1));
    }

    function backwards() {
        selectedDate = selectedDate.plus(navigationAmount(activeView, 1));
    }

    return {
        get date() {
            return selectedDate;
        },
        set date(value: DateTime) {
            selectedDate = value;
        },
        get format() {
            return formatDateTime(this.date, activeView);
        },
        forwards,
        backwards,
    }
}

export function getActiveView() {
    return {
        get view() {
            return activeView;
        },
        set view(v: CalendarView) {
            activeView = v;
        }
    }
}
