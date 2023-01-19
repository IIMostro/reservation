use std::ops::Range;

use chrono::{DateTime, FixedOffset, Utc};
use crate::{convert_to_timestamp, convert_to_utc_time, Reservation, ReservationError, ReservationStatus};

impl Reservation {
    pub fn new_pending(uid: &str, rid: &str, note: &str,
                       start_time: DateTime<FixedOffset>,
                       end_time: DateTime<FixedOffset>) -> Self {
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
            return Err(ReservationError::InvalidResourceId(self.resource_id.clone()));
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