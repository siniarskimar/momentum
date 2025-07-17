use std::sync::Mutex;

use chrono::DateTime;

use crate::{
    app::GlobalState,
    storage::{insert_event, insert_interval, Event, Interval, IntervalQuery, IntervalReference},
};

#[tauri::command]
pub fn get_intervals_between_dates(
    state: tauri::State<Mutex<GlobalState>>,
    begins_at: i64,
    ends_at: i64,
) -> Result<Vec<Interval>, String> {
    let state = state.lock().unwrap();

    let begins_at = DateTime::from_timestamp(begins_at, 0).unwrap();
    let ends_at = DateTime::from_timestamp(ends_at, 0).unwrap();

    return state
        .database
        .select_intervals(IntervalQuery::new().between_dates(&begins_at, &ends_at))
        .map_err(|e| e.to_string());
}

#[tauri::command]
pub fn create_event(
    state: tauri::State<Mutex<GlobalState>>,
    event: Event,
    intervals: Vec<Interval>,
) -> Result<Event, String> {
    let mut state = state.lock().unwrap();

    let event_id = state
        .database
        .transaction(|tx| {
            let event_id = insert_event(tx, &event)?;
            let intervals = intervals.iter().map(|i| {
                let mut i = i.clone();
                i.references = IntervalReference::Event(event_id);
                return i;
            });

            for i in intervals {
                insert_interval(tx, &i)?;
            }

            return Ok(event_id);
        })
        .map_err(|e: rusqlite::Error| e.to_string())?;

    let mut event = event.clone();
    event.id = event_id;
    return Ok(event);
}

#[tauri::command]
pub fn get_events(
    state: tauri::State<Mutex<GlobalState>>,
    ids: Vec<u64>,
) -> Result<Option<Event>, String> {
    let state = state.lock().unwrap();

    return state
        .database
        .select_events(&ids)
        .map(|v| v.first().cloned())
        .map_err(|err| err.to_string());
}
