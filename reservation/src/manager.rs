use async_trait::async_trait;
use sqlx::postgres::types::PgRange;
use abi::{Reservation, ReservationError, ReservationQuery};
use crate::{ReservationId, ReservationManager, Rsvp};
use chrono::{Utc, DateTime};
use sqlx::{PgPool, Row};
use sqlx::types::Uuid;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, ReservationError> {
        rsvp.validate()?;
        let status = abi::ReservationStatus::from_i32(rsvp.status).unwrap_or(abi::ReservationStatus::Pending);
        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan().into();
        let id: Uuid = sqlx::query("insert into rsvp.reservations(user_id, resource_id, timespan, note, status) values($1, $2, $3, $4, $5::rsvp.reservation_status) returning id")
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.notes.clone())
            .bind(status.to_string())
            .fetch_one(&self.pool)
            .await?.get(0);
        rsvp.id  = id.to_string();
        Ok(rsvp)
    }

    async fn change_status(&self, id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(&self, id: ReservationId, note: String) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn get(&self, id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn query(&self, query: ReservationQuery) -> Result<Vec<Reservation>, ReservationError> {
        todo!()
    }
}

impl ReservationManager{
    pub async fn new(pool: PgPool) -> Self {
        Self{pool}
    }
}


#[cfg(test)]
mod tests{
    use chrono::FixedOffset;
    use super::*;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reservation_should_work_for_valid_windows(){
        let manager = ReservationManager::new(migrated_pool.clone()).await;
        let rsvp = Reservation::new_pending("user_id",
                                            "resource_id",
                                            "note",
                                            "2022-12-25T15:00:00-0700".parse().unwrap(),
                                            "2022-12-28T12:00:00-0700".parse().unwrap(),);
        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert_ne!(rsvp.id, "");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reservation_conflict_reservation_should_reject(){
        let manager = ReservationManager::new(migrated_pool.clone()).await;
        let rsvp1 = Reservation::new_pending("alice",
                                            "resource_id",
                                            "note",
                                            "2022-12-25T15:00:00-0700".parse().unwrap(),
                                            "2022-12-28T12:00:00-0700".parse().unwrap(),);
        let rsvp2 = Reservation::new_pending("allen",
                                            "resource_id",
                                            "note",
                                            "2022-12-26T15:00:00-0700".parse().unwrap(),
                                            "2022-12-30T12:00:00-0700".parse().unwrap(),);
        let rsvp = manager.reserve(rsvp1).await.unwrap();
        let rsvp1 = manager.reserve(rsvp2).await.unwrap_err();
        println!("{:?}", rsvp1);
    }
}
