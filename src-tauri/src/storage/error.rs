use serde::{ser::SerializeStruct, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SqliteError(#[from] rusqlite::Error),

    #[error("underlying schema is too old")]
    SchemaTooOld,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut err = serializer.serialize_struct("Error", 1)?;
        err.serialize_field("message", &self.to_string())?;

        err.end()
    }
}
