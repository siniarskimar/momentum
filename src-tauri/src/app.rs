use std::{fmt::Debug, path::PathBuf, sync::Mutex};
use tauri::Manager;

use anyhow::anyhow;

use crate::model::Event;

#[derive(Default, Debug)]
pub struct GlobalState {}

pub fn setup_local_datadir(app: &mut tauri::App) -> anyhow::Result<()> {
    let datadir = get_local_datadir(&app)?;

    let suffixes = &["events", "changelog"];

    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    for &suffix in suffixes {
        let path = datadir.join(suffix);
        println!("{:?}", path);

        match std::fs::metadata(&path) {
            Ok(_) => return Ok(()),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    dir_builder.create(&path)?;
                }
                _ => return Err(anyhow!(err).context("failed to stat local data directory")),
            },
        }
    }

    return Ok(());
}

pub fn get_local_datadir(app: &tauri::App) -> Result<PathBuf, tauri::Error> {
    let product_name = app.config().product_name.clone().unwrap();
    let local_datadir = app.path().local_data_dir()?;
    let mut local_datadir = local_datadir.clone();
    local_datadir.push(product_name);

    return Ok(local_datadir);
}

pub fn load_events_in_range(app: &tauri::App) -> Result<Vec<Event>, std::io::Error> {
    return Ok(vec![]);
}
