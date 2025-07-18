use std::sync::Mutex;
use tauri::Manager;

mod app;
use app::GlobalState;

mod commands;
use commands::*;

mod error;
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
