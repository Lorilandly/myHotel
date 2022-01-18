use super::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct ReservationRepository {
    db: Rc<Connection>,
}

impl Repository for ReservationRepository {}

impl ReservationRepository {
    pub fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }
    pub fn get_reservation(&self, reservation_id: Uuid) -> Result<Reservation> {
        self.db.query_row(
            "SELECT date, room, signin, signout FROM reservations WHERE reservation_id=?1",
            params![reservation_id.to_string()],
            |row| Ok(Reservation{reservation_id,date:row.get(0)?,room:row.get(1)?,checkin:row.get(2)?,checkout:row.get(3)?}),
        )
    }
    pub fn insert_reservation(&self, reservation: &Reservation) -> Result<()> {
        self.db.execute(
            "INSERT INTO reservations (reservation_id, date, room, checkin, checkout) VALUES (?1, ?2, ?3, ?4, ?5)", 
            params![reservation.reservation_id.to_string(),reservation.date,reservation.room,reservation.checkin,reservation.checkout]
        )?;
        Ok(())
    }
    pub fn signin(&self, reservation_id: Uuid) -> Result<()> {
        self.db.execute(
            "UPDATE reservations SET checkin=True WHERE reservation_id=?1",
            params![reservation_id.to_string()]
        )?;
        Ok(())
    }
}
