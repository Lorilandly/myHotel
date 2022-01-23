use crate::*;

pub mod reservation_repository;
pub mod room_repository;

pub trait Repository {
    fn new(db: Rc<Connection>) -> Self;
}
