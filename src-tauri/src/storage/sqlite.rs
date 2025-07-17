use std::path::{Path, PathBuf};

use chrono::DateTime;
use rusqlite::{params, Connection, OptionalExtension, Row, Transaction};

static MIN_VERSION: u32 = 1;
static MAX_VERSION: u32 = 1;
static BASE_SCHEMA: &'static str = include_str!("schema.sql");

pub struct SqliteStorage {
    conn: Connection,
}

struct MasterRow {
    schema_version: u32,
}

impl MasterRow {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
}

impl TryFrom<&rusqlite::Row<'_>> for MasterRow {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            schema_version: row.get("schema_version")?,
        })
    }
}

fn get_master_row(conn: &mut Connection) -> Result<Option<MasterRow>, rusqlite::Error> {
    if !conn.table_exists(None, "_master")? {
        return Ok(None);
    }

    return conn
        .query_row("SELECT * FROM _master", [], |row| MasterRow::try_from(row))
        .optional();
}

/// Produce `count` number of placeholders
pub fn repeat_var(count: usize) -> String {
    let mut s = "?,".repeat(count);
    s.pop();
    return s;
}

impl SqliteStorage {
    pub fn open_or_create(path: &Path) -> Result<Self, rusqlite::Error> {
        let mut conn = Connection::open(path)?;
        let master_row = get_master_row(&mut conn)?;
        if master_row.is_none() {
            let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Exclusive)?;
            tx.execute_batch(BASE_SCHEMA)?;
            tx.commit()?;
        }
        let master_row = get_master_row(&mut conn)?;

        return Ok(Self { conn });
    }

    pub fn open_memory() -> Result<Self, rusqlite::Error> {
        let mut conn = Connection::open_in_memory()?;

        {
            let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Exclusive)?;
            tx.execute_batch(BASE_SCHEMA)?;
            tx.commit()?;
        }

        return Ok(Self { conn });
    }

    pub fn transaction<F, T, E>(&mut self, op: F) -> Result<T, E>
    where
        F: FnOnce(&Transaction) -> Result<T, E>,
        E: std::convert::From<rusqlite::Error>,
    {
        let tx = self.conn.transaction()?;
        let r = op(&tx)?;
        tx.commit()?;
        return Ok(r);
    }

    pub fn query<F, T, E>(&self, op: F) -> Result<T, E>
    where
        F: FnOnce(&Connection) -> Result<T, E>,
        E: std::convert::From<rusqlite::Error>,
    {
        return op(&self.conn);
    }
}
