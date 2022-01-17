use crate::*;

pub struct RoomRepository {
    db: Rc<Connection>,
}

impl RoomRepository {
    pub fn new(db: Rc<Connection>) -> Self {
        Self{db}
    }

    pub fn get_rooms(self) -> Result<Vec<String>>{
        let mut stmt = self.db.prepare("SELECT * FROM room")?;
        let rooms = stmt.query_map([], |row| {
            row.get(0)
        })?.map(|x| x.unwrap()).collect();

        Ok(rooms)
    }
}
