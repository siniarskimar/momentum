use serde::{ser::SerializeStruct, Serialize};
use thiserror;

use crate::storage;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("storage error: {0}")]
    Storage(#[from] storage::error::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Tauri(String),
    Storage(String),
    Io(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let msg = self.to_string();
        let kind = match self {
            Error::Io(_) => ErrorKind::Io(msg),
            Error::Tauri(_) => ErrorKind::Tauri(msg),
            Error::Storage(_) => ErrorKind::Storage(msg),
        };

        kind.serialize(serializer)
    }
}
