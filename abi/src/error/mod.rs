mod conflict;

use chrono::format::Item::Error;
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

pub use conflict::{ReservationConflictInfo, ReservationWindow};

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("unknown error")]
    Unknown,
    #[error("invalid time")]
    InvalidTime,

    #[error("invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Conflict reservation")]
    ConflictReservation(ReservationConflictInfo),

    #[error("invalid resource id: {0}")]
    InvalidResourceId(String),

    #[error("db error")]
    DBError(sqlx::Error),
}

impl From<sqlx::Error> for ReservationError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let error: &PgDatabaseError = e.downcast_ref();
                match (error.code(), error.schema(), error.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => {
                        // detail(Option<&str>) -> unwrap (&str) -> parse(Result<ReservationConflictInfo>) -> unwrap
                        ReservationError::ConflictReservation(
                            error.detail().unwrap().parse().unwrap(),
                        )
                    }
                    _ => ReservationError::DBError(sqlx::Error::Database(e)),
                }
            }
            _ => ReservationError::DBError(e),
        }
    }
}
