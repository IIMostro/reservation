use async_trait::async_trait;
use sqlx::postgres::types::PgRange;
use abi::{Reservation, ReservationQuery};
use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use chrono::{Utc, DateTime, NaiveDateTime};
use sqlx::Row;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, ReservationError> {
        if rsvp.start_time.is_none() || rsvp.end_time.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        // 转换时间格式
        let start_time = abi::convert_to_utc_time(rsvp.start_time.as_ref().unwrap().clone());
        let end_time = abi::convert_to_utc_time(rsvp.end_time.as_ref().unwrap().clone());

        if start_time <= end_time {
            return Err(ReservationError::InvalidTime);
        }

        let timespan: PgRange<DateTime<Utc>> = (start_time..end_time).into();
        let id = sqlx::query("insert into reservation(user_id, resource_id, timespan, note, status) values($1, $2, $3, $4, $5) returning id")
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.notes.clone())
            .bind(rsvp.status.clone())
            .fetch_one(&self.pool)
            .await?.get(0);
        rsvp.id  = id;
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