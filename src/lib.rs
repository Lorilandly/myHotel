mod app;
mod entity;
mod repository;

pub use self::entity::reservation::*;
pub use self::repository::{reservation_repository::*, room_repository::*};
use chrono::naive::*;
use rusqlite::{params, Connection, Result};
use std::error;
use std::rc::Rc;
use uuid::Uuid;

fn start() -> Result<()> {
    let sql = Rc::new(Connection::open("hotel.db")?);
    Ok(())
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
