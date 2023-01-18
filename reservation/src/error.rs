use thiserror::Error;

#[derive(Error, Debug)]
pub enum  ReservationError {

    #[error("unknown error")]
    Unknown,
    #[error("invalid time")]
    InvalidTime,

    #[error("db error")]
    DBError(#[from] sqlx::Error),
}