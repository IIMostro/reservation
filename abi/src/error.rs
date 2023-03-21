use chrono::{DateTime, Utc};
use chrono::format::Item::Error;
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("unknown error")]
    Unknown,
    #[error("invalid time")]
    InvalidTime,

    #[error("invalid user id: {0}")]
    InvalidUserId(String),

    #[error("{0}")]
    ConflictReservation(String),

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
                        ReservationError::ConflictReservation(error.detail().unwrap().to_string())
                    }
                    _ => ReservationError::DBError(sqlx::Error::Database(e))
                }
            }
            _ => ReservationError::DBError(e)
        }
    }
}

// 错误转换
pub struct ReservationConflictInfo{
    a: ReservationWindow,
    b: ReservationWindow,
}

pub struct ReservationWindow{
    rid: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>
}
