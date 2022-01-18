use crate::*;

#[derive(Clone, Debug)]
pub struct ReservationController {
    reservation_repository: ReservationRepository,
    room_repository: RoomRepository,
}

impl ReservationController {
    pub fn new(reservation_repository: ReservationRepository, room_repository: RoomRepository) -> Self {
        Self{reservation_repository, room_repository}
    }
    pub fn reserve(self, checkin_date: &str) -> Result<Uuid, Box<dyn error::Error>> {
        // date needs to be in yyyy-mm-dd
        let checkin_date: NaiveDate = checkin_date.parse()?;

        // get available room, return error if no available
        let rooms = self.room_repository.get_empty_rooms(checkin_date)?;

        // make reservations
        let reservation =
            Reservation::new(checkin_date, rooms.first().ok_or(NoRoomError)?.parse()?);
        self.reservation_repository
            .insert_reservation(&reservation)?;
        Ok(reservation.reservation_id)
    }
}

#[derive(Debug)]
struct NoRoomError;

impl error::Error for NoRoomError {}

impl std::fmt::Display for NoRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No Empty room available for the chosen date")
    }
}
