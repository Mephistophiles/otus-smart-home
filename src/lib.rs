//! This is a prototype library for Smart Home Control

mod device;
mod error;
mod home;
mod room;

pub use crate::{
    device::{Device, Plug, SmartDevice, Thermometer},
    home::Home,
    room::Room,
};
