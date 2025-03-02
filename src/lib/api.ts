import { DateTime } from "luxon";

export function getLocaleString(): string {
    if (!!navigator.languages && navigator.languages.length) {
        return navigator.languages[0];
    }
    return navigator.language;
}
