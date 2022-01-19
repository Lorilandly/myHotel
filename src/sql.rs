use crate::*;

pub struct Sql;

impl Sql {
    pub fn init(db: Rc<Connection>) -> Result<()> {
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
            "CREATE TABLE IF NOT EXIST room (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
            params![],
        )?;
        todo!()
    }
}
