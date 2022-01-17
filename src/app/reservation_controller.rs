use crate::*;
use std::str::FromStr;

pub struct ReservationController{
    reservation_repository: ReservationRepository,
    room_repository: RoomRepository,
}

impl ReservationController {
    fn reserve(self, checkin_date: &str) -> Result<Uuid, Box<dyn error::Error>> {
        let checkin_date: NaiveDate = checkin_date.parse()?;

        // get available room, return error if no available
        RoomRepository.getRooms(checkin_date)?;

        // make reservations
        let reservation = Reservation::new(checkin_date, 1001);
        self.reservation_repository.insert_reservation(&reservation);
        Ok(reservation.reservation_id)
    }
}