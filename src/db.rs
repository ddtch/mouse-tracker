use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ActivityTracker {
    conn: Connection,
}

impl ActivityTracker {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS activity (
                id INTEGER PRIMARY KEY,
                timestamp INTEGER,
                left_clicks INTEGER,
                right_clicks INTEGER,
                movement REAL,
                scrolls INTEGER
            )",
            [],
        )?;
        Ok(ActivityTracker { conn })
    }

    pub fn log_activity(
        &self,
        left_clicks: i32,
        right_clicks: i32,
        movement: f64,
        scrolls: i32,
    ) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_e| rusqlite::Error::ExecuteReturnedResults)? // Map the error
            .as_secs();
        self.conn.execute(
            "INSERT INTO activity (timestamp, left_clicks, right_clicks, movement, scrolls)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![now, left_clicks, right_clicks, movement, scrolls],
        )?;
        Ok(())
    }
}
