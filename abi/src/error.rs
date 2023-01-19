use thiserror::Error;

#[derive(Error, Debug)]
pub enum  ReservationError {

    #[error("unknown error")]
    Unknown,
    #[error("invalid time")]
    InvalidTime,

    #[error("invalid user id: {0}")]
    InvalidUserId(String),

    #[error("invalid resource id: {0}")]
    InvalidResourceId(String),

    #[error("db error")]
    DBError(#[from] sqlx::Error),
}