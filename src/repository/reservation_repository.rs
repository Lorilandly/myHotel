use super::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct ReservationRepository {
    db: Rc<Connection>,
}

impl Repository for ReservationRepository {}

impl ReservationRepository {
    pub fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }

    pub fn get_reservation(&self, reservation_id: &str) -> Result<Reservation> {
        self.db.query_row(
            "SELECT reservation_id, date, room, checkin, checkout FROM reservation WHERE reservation_id LIKE ?1",
            params![format!("{}{}", reservation_id, "%")],
            |row| {
                Ok(Reservation {
                    reservation_id: row.get::<usize, String>(0)?.parse().unwrap(),
                    date: row.get(1)?,
                    room: row.get::<usize, u32>(2)?,
                    checkin: row.get(3)?,
                    checkout: row.get(4)?,
                })
            },
        )
    }
    pub fn insert_reservation(&self, reservation: &Reservation) -> Result<()> {
        self.db.execute(
            "INSERT INTO reservation (reservation_id, date, room) VALUES (?1, date(?2), ?3)",
            params![
                reservation.reservation_id.to_string(),
                reservation.date,
                reservation.room
            ],
        )?;
        Ok(())
    }
    pub fn signin(&self, reservation_id: Uuid) -> Result<()> {
        self.db.execute(
            "UPDATE reservation SET checkin=True WHERE reservation_id=?1",
            params![reservation_id.to_string()],
        )?;
        Ok(())
    }
    pub fn signout(&self, reservation_id: Uuid) -> Result<()> {
        self.db.execute(
            "UPDATE reservation SET checkout=True WHERE reservation_id=?1",
            params![reservation_id.to_string()],
        )?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn conversion() {
        let date: NaiveDate = "1234-1-1".parse().unwrap();
        assert_eq!(format!("{}", date), "1234-01-01")
    }

    #[test]
    fn vector() {
        let rooms = vec!("1002");
        let reservation =
            Reservation::new("1111-1-1".parse::<NaiveDate>().unwrap(), rooms.first().unwrap().parse().unwrap());
        assert_eq!(reservation.room, 1002)
    }
}
