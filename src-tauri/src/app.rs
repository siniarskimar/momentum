use std::path::PathBuf;
use tauri::Manager;

use crate::storage::SqliteStorage;

pub fn setup_local_datadir(app: &mut tauri::App) -> anyhow::Result<()> {
    let datadir = get_local_datadir(&app)?;

    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);
    dir_builder.create(&datadir)?;

    return Ok(());
}

pub fn get_local_datadir(app: &tauri::App) -> Result<PathBuf, tauri::Error> {
    let product_name = app.config().product_name.clone().unwrap();
    let local_datadir = app.path().local_data_dir()?;
    let mut local_datadir = local_datadir.clone();
    local_datadir.push(product_name);

    return Ok(local_datadir);
}

pub struct GlobalState {
    pub database: SqliteStorage,
}

impl GlobalState {
    pub fn new(app: &mut tauri::App) -> anyhow::Result<Self> {
        let mut datadir = get_local_datadir(app)?;
        datadir.push("calendar.db");

        let database = SqliteStorage::open_or_create(&datadir)?;
        return Ok(Self { database });
    }
}
