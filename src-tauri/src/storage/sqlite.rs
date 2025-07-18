use std::path::Path;

use crate::storage::error;

use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior};

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

fn get_master_row(conn: &Connection) -> Result<Option<MasterRow>, rusqlite::Error> {
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

fn setup_base_schema(conn: &mut Connection) -> Result<MasterRow, rusqlite::Error> {
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Exclusive)?;
    tx.execute_batch(BASE_SCHEMA)?;
    tx.commit()?;
    return Ok(get_master_row(conn)?.unwrap());
}

fn upgrade_schema(_conn: &mut Connection, _current_version: u32) -> Result<(), rusqlite::Error> {
    return Ok(());
}

impl SqliteStorage {
    pub fn open_or_create(path: &Path) -> error::Result<Self> {
        let mut conn = Connection::open(path)?;
        let mut master_row = get_master_row(&mut conn)?;
        if master_row.is_none() {
            master_row = Some(setup_base_schema(&mut conn)?);
        }

        let master_row = master_row.unwrap();
        let schema_version = master_row.schema_version();
        if schema_version > MIN_VERSION && schema_version < MAX_VERSION {
            upgrade_schema(&mut conn, schema_version)?;
        } else if schema_version < MIN_VERSION {
            return Err(error::Error::SchemaTooOld);
        } else
        /* if schema_version > MAX_VERSION */
        {
            return Err(error::Error::Corrupted(String::from(
                "schema version too new",
            )));
        }

        return Ok(Self { conn });
    }

    pub fn open_memory() -> Result<Self, rusqlite::Error> {
        let mut conn = Connection::open_in_memory()?;
        let master = setup_base_schema(&mut conn)?;
        upgrade_schema(&mut conn, master.schema_version())?;

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

    pub fn transaction_with_behavior<F, T, E>(
        &mut self,
        behavior: TransactionBehavior,
        op: F,
    ) -> Result<T, E>
    where
        F: FnOnce(&Transaction) -> Result<T, E>,
        E: std::convert::From<rusqlite::Error>,
    {
        let tx = self.conn.transaction_with_behavior(behavior)?;
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
