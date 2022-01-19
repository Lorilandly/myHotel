mod app;
mod entity;
mod repository;
mod sql;

pub use self::app::{checkin_controller::*, checkout_controller::*, reservation_controller::*};
pub use self::entity::reservation::*;
pub use self::repository::{reservation_repository::*, room_repository::*};
use self::sql::*;
use chrono::naive::*;
use rusqlite::{params, Connection, Result};
use std::error;
use std::rc::Rc;
use uuid::Uuid;

pub struct Hotel {
    reservation_controller: ReservationController,
    checkin_controller: CheckinController,
    checkout_controller: CheckoutController,
}

impl Hotel {
    pub fn new() -> Result<Self> {
        let db = Rc::new(Connection::open("hotel.db")?);
        Sql::init(db.clone());
        let reservation_repo = ReservationRepository::new(db.clone());
        let room_repo = RoomRepository::new(db.clone());
        let reservation_controller =
            ReservationController::new(reservation_repo.clone(), room_repo.clone());
        let checkin_controller = CheckinController::new(reservation_repo.clone());
        let checkout_controller = CheckoutController::new(reservation_repo.clone());
        Ok(Self {
            reservation_controller,
            checkin_controller,
            checkout_controller,
        })
    }
    pub fn reserve(&self, date: &str) -> Option<String> {
        match self.reservation_controller.reserve(date) {
            Ok(i) => Some(i.to_string()),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn reservation() {
        let a = Hotel::new().unwrap();
        let b = a.checkout_controller.checkout("1".to_string(), "b3127".to_string());
        // let b = a.checkin_controller.checkin("b3127".to_string());
        // let b = a.reservation_controller.reserve("1111-1-1");

        println!("{:?}", b);
    }
}
