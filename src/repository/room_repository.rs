use crate::*;

#[derive(Clone, Debug)]
pub(crate) struct RoomRepository {
    db: Rc<Connection>,
}

impl repository::Repository for RoomRepository {}

impl RoomRepository {
    pub(crate) fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }

    pub(crate) fn get_empty_rooms(&self, date: NaiveDate) -> Result<Vec<String>> {
        let mut stmt = self.db.prepare(&format!(
            "SELECT room_number FROM room
                WHERE room_number NOT IN (
                    SELECT room FROM reservation WHERE date=date('{}')
                )",
            date.to_string(),
        ))?;
        let rooms: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .map(|x| x.unwrap())
            .collect();

        Ok(rooms)
    }
}
