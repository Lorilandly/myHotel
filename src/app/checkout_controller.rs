use crate::*;

pub(crate) struct CheckoutController {
    reservation_repository: ReservationRepository,
}

impl CheckoutController {
    pub(crate) fn new(reservation_repository: ReservationRepository) -> Self {
        Self {
            reservation_repository,
        }
    }

    #[allow(unused_variables)]
    pub(crate) fn checkout(
        &self,
        room: String,
        reservation_id: String,
    ) -> Result<(), Box<dyn error::Error>> {
        // get reservation from id
        let reservation = self
            .reservation_repository
            .get_reservation(&reservation_id)?;

        // check if checked in
        if reservation.checkin == false {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Customer hasn't checked in yet!",
            )));
        }

        // switch toggle state for checkout
        self.reservation_repository
            .checkout(reservation.reservation_id)?;
        Ok(())
    }
}
