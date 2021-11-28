use crate::{Device, Room};
use thiserror::Error;

/// Error kind
#[derive(Error, Debug)]
pub enum Error {
    #[error("Device is already exists")]
    DeviceAlreadyExists(Device),

    #[error("Room is already exists")]
    RoomAlreadyExists(Room),

    #[error("Not yet implemented")]
    NotImplemented,
}

/// Predefined result with Error
pub type Result<T> = std::result::Result<T, Error>;
