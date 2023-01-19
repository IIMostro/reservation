mod manager;

use async_trait::async_trait;
use abi::ReservationError;

pub type ReservationId = String;

#[derive(Debug, Clone)]
pub struct ReservationManager{

    pool: sqlx::PgPool,

}

#[async_trait]
pub trait Rsvp{

    /// 创建一个reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;

    /// 修改一个reservation的状态 pending -> confirmed
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;

    /// update note
    async fn update_note(&self, id: ReservationId, note: String) -> Result<abi::Reservation, ReservationError>;

    /// delete a reservation
    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError>;

    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;

    /// query reservation
    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, ReservationError>;
}