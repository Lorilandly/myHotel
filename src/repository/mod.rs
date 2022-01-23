use crate::*;

mod reservation_repository;
mod room_repository;

pub(crate) use self::{reservation_repository::*, room_repository::*};

pub trait Repository {
    fn new(db: Rc<Connection>) -> Self;
}
