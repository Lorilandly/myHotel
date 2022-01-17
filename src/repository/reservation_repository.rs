use crate::*;
use super::*;

pub struct ReservationRepository{
    db: Rc<Connection>,
}

impl Repository for ReservationRepository{}

impl ReservationRepository{
    pub fn new(db: Rc<Connection>) -> Self{
        Self{db}
    }
    pub fn insert_reservation(self, reservation: &Reservation){
        self.db.execute("some cmd", []);
    }
}