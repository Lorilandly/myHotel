use crate::*;

pub struct CheckoutController {
    reservation_repository: ReservationRepository,
}

impl CheckoutController {
    pub fn new(reservation_repository: ReservationRepository) -> Self {
        Self{reservation_repository}
    }

    pub fn checkout(&self, room: String, reservation_id: Uuid) -> Result<(), Box<dyn error::Error>> {
        // get reservation from id
        let reservation = self.reservation_repository.get_reservation(reservation_id)?;

        // check if checked in
        if reservation.checkin == true {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Customer not checked in yet!")))
        }

        // switch toggle state for checkout
        self.reservation_repository.signout(reservation_id)?;
        Ok(())
    }
}