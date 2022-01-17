use crate::*;

#[derive(Clone, Debug)]
pub struct Reservation {
    pub reservation_id: Uuid,
    pub date: NaiveDate,
    pub room: u32,
    pub checkin: bool,
    pub checkout: bool,
}

impl Reservation {
    pub fn new(date: NaiveDate, room: u32) -> Self {
        Self {
            reservation_id: Uuid::new_v4(),
            date,
            room,
            checkin: false,
            checkout: false
        }
    }
}
