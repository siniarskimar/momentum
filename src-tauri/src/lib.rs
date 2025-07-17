use std::sync::Mutex;
use tauri::{Manager, State};

mod app;
use app::GlobalState;

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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
