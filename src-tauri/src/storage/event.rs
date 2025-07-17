use chrono::format;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::storage::{sqlite::repeat_var, SqliteStorage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: u64,
    pub title: String,
}

fn build_select(ids: &[u64]) -> String {
    if ids.is_empty() {
        return String::from("SELECT id, title FROM event");
    }
    return format!(
        "SELECT id, title FROM event WHERE id IN ({})",
        repeat_var(ids.len())
    );
}

pub fn select_events(conn: &Connection, ids: &[u64]) -> Result<Vec<Event>, rusqlite::Error> {
    let mut stmt = conn.prepare(&build_select(ids))?;

    return stmt
        .query(rusqlite::params_from_iter(ids.iter()))?
        .mapped(|row| {
            Ok(Event {
                id: row.get("id")?,
                title: row.get("title")?,
            })
        })
        .collect();
}

pub fn insert_event(conn: &Connection, event: &Event) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare_cached(
        "
            INSERT INTO event(title)
            VALUES (?)
            RETURNING id
        ",
    )?;
    let id = stmt.query_one(params![event.title], |row| row.get::<_, u64>("id"))?;

    return Ok(id);
}

pub fn insert_event_full(conn: &Connection, event: &Event) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare_cached(
        "
            INSERT INTO event(id, title)
            VALUES (?, ?)
        ",
    )?;
    stmt.execute(params![event.id, event.title])?;
    return Ok(());
}

impl SqliteStorage {
    pub fn select_events(&self, ids: &[u64]) -> Result<Vec<Event>, rusqlite::Error> {
        return self.query(|conn| select_events(conn, ids));
    }

    pub fn insert_event_full(&mut self, event: &Event) -> Result<(), rusqlite::Error> {
        return self.query(|conn| insert_event_full(conn, event));
    }

    pub fn insert_event(&mut self, event: &Event) -> Result<u64, rusqlite::Error> {
        return self.query(|conn| insert_event(conn, event));
    }
}
