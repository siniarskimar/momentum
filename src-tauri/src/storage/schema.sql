CREATE TABLE _master (
    schema_version INTEGER NOT NULL
);

CREATE TABLE interval (
    id INTEGER PRIMARY KEY,
    start_at INTEGER NOT NULL,
    end_at INTEGER NOT NULL,

    ref_type INTEGER NOT NULL,
    ref_id INTEGER NOT NULL,

    -- 1 = event
    -- 2 = task
    -- 3 = reminder
    CHECK (ref_type IN (1, 2, 3)),
    UNIQUE (start_at, end_at, ref_type, ref_id)
);

CREATE TABLE event (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE task (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE reminder (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL  
);


INSERT INTO _master
VALUES (
    1
);
