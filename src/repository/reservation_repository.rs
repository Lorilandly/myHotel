use super::*;
use crate::*;

pub struct ReservationRepository {
    db: Rc<Connection>,
}

impl Repository for ReservationRepository {}

impl ReservationRepository {
    pub fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }
    pub fn insert_reservation(self, reservation: &Reservation) -> Result<()> {
        self.db.execute(
            "INSERT INTO reservations (reservation_id, date, room, checkin, checkout) VALUES (?1, ?2, ?3, ?4, ?5)", 
            params![reservation.reservation_id.to_string(),reservation.date,reservation.room,reservation.checkin,reservation.checkout]
        )?;
        Ok(())
    }
}
