export type CalendarView = "month" | "week" | "day";

export function getLocaleString(): string {
    if (!!navigator.languages && navigator.languages.length) {
        return navigator.languages[0];
    }
    return navigator.language;
}


export function getDateTimeFormatOptions(view: CalendarView): Intl.DateTimeFormatOptions {
    const options = {
        month: {
            month: "long",
            year: "numeric",
        },
        week: {
            month: "long",
            year: "numeric",
            day: "numeric",
        },
        day: {
            month: "long",
            year: "numeric",
            day: "numeric",
        }
    };

    return options[view] as Intl.DateTimeFormatOptions;
}
