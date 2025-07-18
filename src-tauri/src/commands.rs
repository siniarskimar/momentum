use std::sync::Mutex;

use chrono::DateTime;

use crate::{
    app::GlobalState,
    error::Error,
    storage::{insert_event, insert_interval, Event, Interval, IntervalQuery, IntervalReference},
};

use crate::error::Result;

#[tauri::command]
pub fn get_intervals_between_dates(
    state: tauri::State<Mutex<GlobalState>>,
    begins_at: i64,
    ends_at: i64,
) -> Result<Vec<Interval>> {
    let state = state.lock().unwrap();

    let begins_at = DateTime::from_timestamp(begins_at, 0).unwrap();
    let ends_at = DateTime::from_timestamp(ends_at, 0).unwrap();

    return Ok(state
        .database
        .select_intervals(IntervalQuery::new().between_dates(&begins_at, &ends_at))?);
}

#[tauri::command]
pub fn create_event(
    state: tauri::State<Mutex<GlobalState>>,
    event: Event,
    intervals: Vec<Interval>,
) -> Result<Event> {
    if intervals.is_empty() {
        return Err(Error::CommandValidation(String::from(
            "can't create event with no timespans",
        )));
    }
    let mut state = state.lock().unwrap();

    let event_id = state.database.transaction(|tx| -> Result<u64> {
        let event_id = insert_event(tx, &event)?;

        for i in intervals {
            let mut i = i.clone();
            i.references = IntervalReference::Event(event_id);
            insert_interval(tx, &i)?;
        }

        return Ok(event_id);
    })?;

    let mut event = event.clone();
    event.id = event_id;
    return Ok(event);
}

#[tauri::command]
pub fn get_events(state: tauri::State<Mutex<GlobalState>>, ids: Vec<u64>) -> Result<Option<Event>> {
    let state = state.lock().unwrap();

    return Ok(state
        .database
        .select_events(&ids)
        .map(|v| v.first().cloned())?);
}
