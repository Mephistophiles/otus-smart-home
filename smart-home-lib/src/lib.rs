//! This is a prototype library for Smart Home Control

mod device;
pub mod error;
mod home;
mod room;

type Name = String;

use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use error::{Error, Result};

// use self::device::hardcoded_devices::{ExampleSocket, ExampleThermometer};
pub use crate::{
    device::{Device, SmartDevice, SmartSocket, SmartThermometer},
    home::Home,
    room::Room,
};

#[derive(Default, Debug)]
pub struct SmartHub {
    home_list: HashMap<Name, Home>,
}

impl SmartHub {
    pub fn new() -> Self {
        // let mut hub = Self::default();

        // hub.register_socket_impl("example", ExampleSocket);
        // hub.register_thermometer_impl("example", ExampleThermometer);

        // hub
        Self::default()
    }

    pub fn add_home(&mut self, home: Home) -> Result<&mut Home> {
        match self.home_list.entry(home.name().to_string()) {
            Occupied(_) => Err(Error::HomeAlreadyExists(home)),
            Vacant(entry) => Ok(entry.insert(home)),
        }
    }

    pub fn del_home(&mut self, name: &str) -> Option<Home> {
        self.home_list.remove(name)
    }

    pub fn get_home(&self, name: &str) -> Option<&Home> {
        self.home_list.get(name)
    }

    pub fn get_home_mut(&mut self, name: &str) -> Option<&mut Home> {
        self.home_list.get_mut(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Home> {
        self.home_list.values()
    }
}
