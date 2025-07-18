pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlite error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("underlying schema is too old")]
    SchemaTooOld,

    #[error("calendar database likely corrupted: {0}")]
    Corrupted(String),
}
