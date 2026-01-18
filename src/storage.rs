use crate::story::Story;
use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS favorites (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT,
    by TEXT NOT NULL,
    score INTEGER NOT NULL,
    time INTEGER NOT NULL,
    descendants INTEGER,
    saved_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_time ON favorites(time DESC);
"#;

pub struct FavoritesDB {
    conn: Connection,
}

impl FavoritesDB {
    pub fn new() -> SqliteResult<Self> {
        let db_path = Self::db_path();
        let parent_dir = db_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).map_err(|e| {
            rusqlite::Error::ToSqlConversionFailure(Box::new(e))
        })?;

        let conn = Connection::open(db_path)?;
        conn.execute_batch(SCHEMA)?;

        Ok(Self { conn })
    }

    fn db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("my_egui_pro");
        path.push("favorites.db");
        path
    }

    pub fn add_favorite(&self, story: &Story) -> SqliteResult<()> {
        let saved_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn.execute(
            "INSERT OR REPLACE INTO favorites (id, title, url, by, score, time, descendants, saved_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &(story.id as i64),
                &story.title,
                &story.url,
                &story.by,
                &story.score,
                &(story.time as i64),
                &story.descendants,
                &saved_at,
            ),
        )?;

        Ok(())
    }

    pub fn remove_favorite(&self, id: u64) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM favorites WHERE id = ?1", [&(id as i64)])?;
        Ok(())
    }

    pub fn is_favorite(&self, id: u64) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM favorites WHERE id = ?1")?;
        let count: i64 = stmt.query_row([&(id as i64)], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_all(&self) -> SqliteResult<Vec<Story>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, url, by, score, time, descendants
             FROM favorites
             ORDER BY time DESC",
        )?;

        let stories = stmt.query_map([], |row| {
            Ok(Story {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                by: row.get(3)?,
                score: row.get(4)?,
                time: row.get(5)?,
                descendants: row.get(6)?,
            })
        })?;

        stories.collect()
    }
}
