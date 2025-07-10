use std::sync::Mutex;
use tauri::Manager;

mod app;
use app::GlobalState;

mod hex;
mod model;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(GlobalState::default()));
            app::setup_local_datadir(app)?;
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
