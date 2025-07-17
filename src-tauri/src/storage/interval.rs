use chrono::Utc;
use rusqlite::{named_params, params, Connection};
use serde::{Deserialize, Serialize};

use crate::storage::SqliteStorage;

#[derive(Clone, Serialize, Deserialize)]
pub struct Interval {
    pub id: u64,
    pub begins_at: i64,
    pub ends_at: i64,

    pub references: IntervalReference,
}

impl TryFrom<&rusqlite::Row<'_>> for Interval {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Interval {
            id: row.get("id")?,
            begins_at: row.get("start_at")?,
            ends_at: row.get("end_at")?,
            references: IntervalReference::from_tag(row.get("ref_type")?, row.get("ref_id")?)
                .expect("database should have valid interval references"), // TODO: replace this with error return
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum IntervalReference {
    Event(u64),
    Task(u64),
    Reminder(u64),
}

impl IntervalReference {
    pub const fn from_tag(tag: u32, id: u64) -> Option<Self> {
        match tag {
            1 => Some(IntervalReference::Event(id)),
            2 => Some(IntervalReference::Task(id)),
            3 => Some(IntervalReference::Reminder(id)),
            _ => None,
        }
    }

    pub const fn tag(&self) -> u32 {
        match self {
            IntervalReference::Event(_) => 1,
            IntervalReference::Task(_) => 2,
            IntervalReference::Reminder(_) => 3,
        }
    }

    pub const fn value(&self) -> u64 {
        match self {
            IntervalReference::Event(id) => *id,
            IntervalReference::Task(id) => *id,
            IntervalReference::Reminder(id) => *id,
        }
    }
}

#[derive(Clone)]
pub struct IntervalQuery {
    id: Option<u64>,
    begins_at: Option<i64>,
    ends_at: Option<i64>,
    references: Option<IntervalReference>,
}

impl IntervalQuery {
    pub fn new() -> Self {
        Self {
            id: None,
            begins_at: None,
            ends_at: None,
            references: None,
        }
    }

    pub fn with_id(&mut self, id: u64) -> &mut Self {
        self.id = Some(id);
        return self;
    }

    pub fn between_dates(
        &mut self,
        begins: &chrono::DateTime<Utc>,
        ends: &chrono::DateTime<Utc>,
    ) -> &mut Self {
        self.begins_at = Some(begins.timestamp());
        self.ends_at = Some(ends.timestamp());
        return self;
    }

    pub fn with_reference(&mut self, reference: IntervalReference) -> &mut Self {
        self.references = Some(reference);
        return self;
    }

    fn build(&self) -> String {
        let stmt = "SELECT * FROM interval";
        let mut conditions: Vec<&'static str> = Vec::new();

        if self.id.is_some() {
            conditions.push("id = :id");
        }

        if self.begins_at.is_some() {
            conditions.push("start_at = :start_at AND end_at = :end_at");
        }

        if self.references.is_some() {
            conditions.push("ref_type = :ref_type AND ref_id = :ref_id");
        }

        if conditions.is_empty() {
            return stmt.to_string();
        }

        let stmt = [stmt, " WHERE ", &conditions.join(" AND ")].join("");
        return stmt;
    }
}

pub fn select_intervals(
    conn: &Connection,
    query: &IntervalQuery,
) -> Result<Vec<Interval>, rusqlite::Error> {
    let mut stmt = conn.prepare_cached(&query.build())?;
    return stmt
        .query(named_params! {
            ":id": query.id,
            ":start_at": query.begins_at,
            ":end_at": query.ends_at,
            ":ref_type": query.references.as_ref().and_then(|x| Some(x.tag())),
            ":ref_id": query.references.as_ref().and_then(|x| Some(x.value()))
        })?
        .mapped(|row| Interval::try_from(row))
        .collect();
}

pub fn insert_interval(conn: &Connection, interval: &Interval) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare_cached(
        "
            INSERT INTO interval(start_at, end_at, ref_type, ref_id)
            VALUES (?, ?, ?, ?)
            RETURNING id
        ",
    )?;
    let id = stmt.query_one(
        params![
            interval.begins_at,
            interval.ends_at,
            interval.references.tag(),
            interval.references.value(),
        ],
        |row| row.get::<_, u64>("id"),
    )?;

    return Ok(id);
}

impl SqliteStorage {
    pub fn select_intervals(
        &self,
        query: &IntervalQuery,
    ) -> Result<Vec<Interval>, rusqlite::Error> {
        return self.query(|conn| select_intervals(conn, query));
    }

    pub fn insert_interval(&mut self, interval: &Interval) -> Result<u64, rusqlite::Error> {
        return self.query(|conn| insert_interval(conn, interval));
    }
}
