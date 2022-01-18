mod app;
mod entity;
mod repository;

pub use self::app::{reservation_controller::*, checkin_controller::*};
pub use self::entity::reservation::*;
pub use self::repository::{reservation_repository::*, room_repository::*};
use chrono::naive::*;
use rusqlite::{params, Connection, Result};
use std::error;
use std::rc::Rc;
use uuid::Uuid;

pub struct Hotel {
    reservation_controller: ReservationController,
    checkin_controller: CheckinController,
}

impl Hotel {
    fn new() -> Result<Self> {
        let db = Rc::new(Connection::open("hotel.db")?);
        let reservation_repo = ReservationRepository::new(db.clone());
        let room_repo = RoomRepository::new(db.clone());
        let reservation_controller = ReservationController::new(reservation_repo.clone(), room_repo.clone());
        let checkin_controller = CheckinController::new(reservation_repo.clone());
        Ok(Self{reservation_controller, checkin_controller})
    }
}
/*
#[macro_use]
extern crate lazy_static;
lazy_static!{
    static ref CONN: Connection = Connection::open("hotel.db");
}
*/

/*
fn checkin(reservation: &str) -> Result<&str, Box<dyn error::Error>> {
    let date = reservation.parse::<Reservation>()?.free();
}
fn checkout(room: &str) -> Result<&str, Box<dyn error::Error>> {
    todo!()
}
*/
