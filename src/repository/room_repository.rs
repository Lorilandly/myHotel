use crate::*;

#[derive(Clone, Debug)]
pub struct RoomRepository {
    db: Rc<Connection>,
}

impl RoomRepository {
    pub fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }

    pub fn get_empty_rooms(&self, date: NaiveDate) -> Result<Vec<String>> {
        let mut stmt = self
            .db
            // .prepare(&format!("SELECT room FROM reservation where date=date('{}')", date))?;
            .prepare("SELECT * FROM reservation ")?;
        let rooms: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .map(|x| x.unwrap())
            .collect();

        Ok(rooms)
    }
}
