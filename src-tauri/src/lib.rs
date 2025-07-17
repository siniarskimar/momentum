use chrono::DateTime;
use std::sync::Mutex;
use tauri::{Manager, State};

mod app;
use app::GlobalState;

use crate::storage::{
    insert_event, insert_interval, Event, Interval, IntervalQuery, IntervalReference,
};

mod hex;
mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app::setup_local_datadir(app)?;

            let global_state = GlobalState::new(app)?;
            app.manage(Mutex::new(global_state));
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            get_intervals_between_dates,
            create_event,
            get_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_intervals_between_dates(
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
fn create_event(
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
fn get_events(
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
