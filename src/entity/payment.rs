use crate::Entity;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Payment {
    room_number: u32,
    room_type: String,
    room_price: u32,
}

impl Entity for Payment {}
