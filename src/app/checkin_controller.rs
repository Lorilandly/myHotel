use crate::*;

pub(crate) struct CheckinController {
    reservation_repository: ReservationRepository,
}

impl CheckinController {
    pub(crate) fn new(reservation_repository: ReservationRepository) -> Self {
        Self {
            reservation_repository,
        }
    }

    pub(crate) fn checkin(&self, reservation_id: String) -> Result<String> {
        // get reservation from id
        let reservation = self
            .reservation_repository
            .get_reservation(&reservation_id)?;

        // set reservation signin status
        self.reservation_repository
            .checkin(reservation.reservation_id)?;

        // return room number
        Ok(reservation.room.to_string())
    }
}
