use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

pub fn db_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("LCSRS_DB") {
        return Ok(PathBuf::from(p));
    }
    let dirs =
        ProjectDirs::from("", "", "lcsrs").context("could not determine XDG data directory")?;
    let dir = dirs.data_dir();
    std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    Ok(dir.join("lcsrs.db"))
}

pub fn open() -> Result<Connection> {
    let path = db_path()?;
    let mut conn = Connection::open(&path)
        .with_context(|| format!("opening database at {}", path.display()))?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrations().to_latest(&mut conn)?;
    Ok(conn)
}

fn migrations() -> Migrations<'static> {
    Migrations::new(vec![M::up(
        r#"
        CREATE TABLE cards (
            id           INTEGER PRIMARY KEY,
            title        TEXT NOT NULL,
            url          TEXT NOT NULL UNIQUE,
            difficulty   TEXT CHECK (difficulty IN ('easy', 'medium', 'hard')),
            created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            due          TIMESTAMP NOT NULL,
            stability    REAL NOT NULL,
            difficulty_f REAL NOT NULL,
            last_review  TIMESTAMP,
            reps         INTEGER NOT NULL DEFAULT 0,
            lapses       INTEGER NOT NULL DEFAULT 0,
            state        TEXT NOT NULL
        );

        CREATE TABLE tags (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL UNIQUE
        );

        CREATE TABLE card_tags (
            card_id  INTEGER NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
            tag_id   INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (card_id, tag_id)
        );

        CREATE TABLE reviews (
            id            INTEGER PRIMARY KEY,
            card_id       INTEGER NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
            rating        TEXT NOT NULL CHECK (rating IN ('again', 'hard', 'good', 'easy')),
            reviewed_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX idx_cards_due ON cards(due);
        "#,
    )])
}
