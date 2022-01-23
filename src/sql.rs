use crate::*;

pub(super) struct Sql;

impl Sql {
    pub(super) fn init(db_file: &str) -> Result<Connection> {
        if std::path::Path::new(db_file).exists() {
            return Ok(Connection::open(db_file)?);
        }
        let db = Connection::open(db_file)?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS reservation (
                  reservation_id   TEXT PRIMARY KEY,
                  date             REAL NOT NULL,
                  room             INTEGER NOT NULL,
                  checkin          INTEGER DEFAULT 0,
                  checkout         INTEGER DEFAULT 0
                  )",
            params![],
        )?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS room (
                  room_number       TEXT PRIMARY KEY,
                  room_type         TEXT,
                  room_price        INTEGER
                  )",
            params![],
        )?;
        db.execute(
            "INSERT INTO room (room_number) VALUES
                (1001),
                (1002)
            ",
            params![],
        )?;
        Ok(db)
    }
}
