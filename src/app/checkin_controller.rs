use crate::*;

pub struct CheckinController {
    reservation_repository: ReservationRepository,
}

impl CheckinController {
    pub fn new(reservation_repository: ReservationRepository) -> Self {
        Self{reservation_repository}
    }

    pub fn checkin(self, reservation_id: Uuid) -> Result<String> {
        // get reservation from id
        let reservation = self.reservation_repository.get_reservation(reservation_id)?;

        // set reservation signin status
        self.reservation_repository.signin(reservation_id)?;

        // return room number
        Ok(reservation.room.to_string())
    }
}