use thiserror::Error;

use crate::{Device, Home, Room};

/// Error kind
#[derive(Error, Debug)]
pub enum Error {
    #[error("Socket implementation is already registered on this name")]
    SocketImplAlreadyRegistered,

    #[error("Socket implementation is not found")]
    SocketImplNotFound,

    #[error("Thermometer implementation is already registered on this name")]
    ThermometerImplAlreadyRegistered,

    #[error("Thermometer implementation is not found")]
    ThermometerImplNotFound,

    #[error("Home is already exists")]
    HomeAlreadyExists(Home),

    #[error("Device is already exists")]
    DeviceAlreadyExists(Device),

    #[error("Room is already exists")]
    RoomAlreadyExists(Room),

    #[error("Not yet implemented")]
    NotImplemented,

    #[error("Internal error")]
    InternalError,
}

/// Predefined result with Error
pub type Result<T> = std::result::Result<T, Error>;
