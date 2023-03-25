use std::collections::Bound;
use std::ops::Range;

use crate::{
    convert_to_timestamp, convert_to_utc_time, Reservation, ReservationError, ReservationStatus,
};
use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{Error, FromRow, Postgres, Row};
use sqlx::encode::IsNull::No;
use sqlx::postgres::PgRow;
use sqlx::postgres::types::PgRange;

impl Reservation {
    pub fn new_pending(
        uid: &str,
        rid: &str,
        note: &str,
        start_time: DateTime<FixedOffset>,
        end_time: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            id: "".to_string(),
            user_id: uid.to_string(),
            resource_id: rid.to_string(),
            start_time: Some(convert_to_timestamp(start_time.with_timezone(&Utc))),
            end_time: Some(convert_to_timestamp(end_time.with_timezone(&Utc))),
            notes: note.to_string(),
            status: ReservationStatus::Pending as i32,
        }
    }

    pub fn validate(&self) -> Result<(), ReservationError> {
        if self.user_id.is_empty() {
            return Err(ReservationError::InvalidUserId(self.user_id.clone()));
        }

        if self.resource_id.is_empty() {
            return Err(ReservationError::InvalidResourceId(
                self.resource_id.clone(),
            ));
        }

        if self.start_time.is_none() || self.end_time.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        let start_time = convert_to_utc_time(self.start_time.as_ref().unwrap().clone());
        let end_time = convert_to_utc_time(self.end_time.as_ref().unwrap().clone());

        if start_time >= end_time {
            return Err(ReservationError::InvalidTime);
        }
        Ok(())
    }

    pub fn get_timespan(&self) -> Range<DateTime<Utc>> {
        let start_time = convert_to_utc_time(self.start_time.as_ref().unwrap().clone());
        let end_time = convert_to_utc_time(self.end_time.as_ref().unwrap().clone());
        Range {
            start: start_time,
            end: end_time,
        }
    }
}

// 从pgrow转化为reservation
impl FromRow<'_, PgRow> for Reservation{

    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let range: PgRange<DateTime<Utc>> = row.get("timespan");
        let range: NaiveRange<DateTime<Utc>> = range.into();
        assert!(range.start.is_some());
        let start = range.start.unwrap();
        assert!(range.end.is_some());
        let end = range.end.unwrap();
        Ok(Self{
            id: row.get("id"),
            user_id: row.get("user_id"),
            resource_id: row.get("resource_id"),
            start_time: Some(convert_to_timestamp(start)),
            end_time: Some(convert_to_timestamp(end)),
            notes: row.get("note"),
            status: row.get("status")
        })
    }
}

struct NaiveRange<T>{
    start: Option<T>,
    end: Option<T>
}

impl <T> From<PgRange<T>> for NaiveRange<T> {

    fn from(value: PgRange<T>) -> Self {
        let function = |b: Bound<T>| match b {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None
        };
        let start = function(value.start);
        let end = function(value.end);
        Self{ start, end }
    }
}