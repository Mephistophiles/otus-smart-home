//! This is a prototype library for Smart Home Control

mod device;
pub mod error;
mod home;
mod room;

pub use crate::{
    device::{Device, SmartDevice, SmartSocket, SmartThermometer},
    home::Home,
    room::Room,
};
