mod app;
mod entity;
mod gui;
mod repository;
mod sql;

use self::app::*;
use self::entity::*;
use self::gui::*;
use self::repository::*;
use self::sql::*;
use chrono::naive::*;
use rusqlite::{params, Connection, Result};
use std::error;
use std::rc::Rc;
use uuid::Uuid;

pub fn main() -> iced::Result {
    HotelUI::run(Settings::default())
}

pub struct Hotel {
    reservation_controller: ReservationController,
    checkin_controller: CheckinController,
    checkout_controller: CheckoutController,
}

impl Hotel {
    pub fn new() -> Result<Self> {
        let db = Rc::new(Sql::init(&"hotel.db")?);
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
    pub fn reserve(&self, date: &str) -> Result<String, String> {
        match self.reservation_controller.reserve(date) {
            Ok(i) => Ok(i.to_string()),
            Err(e) => Err(format!("{}", e)),
        }
    }
    pub fn cancel(&self, reservation_id: &str) -> Result<String, String> {
        match self.reservation_controller.cancel(reservation_id) {
            Ok(_) => Ok(String::from("success")),
            Err(e) => Err(format!("{}", e)),
        }
    }
    pub fn checkin(&self, reservation_id: &str) -> Result<String, String> {
        match self.checkin_controller.checkin(reservation_id) {
            Ok(i) => Ok(i.to_string()),
            Err(e) => Err(format!("{}", e)),
        }
    }
    pub fn checkout(&self, reservation_id: &str, room: &str) -> Result<String, String> {
        match self.checkout_controller.checkout(room, reservation_id) {
            Ok(_) => Ok(String::from("Success")),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn reservation() {
        let a = Hotel::new().expect("sth");
        // let b = a.reserve("1111-1-1");
        let b = a.cancel("547".to_string());
        // let b = a.checkin("4a7".to_string());
        // let b = a.checkout(String::from("1"), "1bd8".to_string());

        println!("{:?}", b);
    }
}
