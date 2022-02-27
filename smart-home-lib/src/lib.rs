//! This is a prototype library for Smart Home Control

mod device;
pub mod error;
mod home;
mod room;

type Name = String;

use std::collections::{
    hash_map::Entry::{self, Occupied, Vacant},
    HashMap,
};

use error::{Error, Result};

// use self::device::hardcoded_devices::{ExampleSocket, ExampleThermometer};
pub use crate::{
    device::{Device, SmartDevice, SmartSocket, SmartThermometer},
    home::Home,
    room::Room,
};

#[derive(Default)]
pub struct SmartHub {
    home_list: HashMap<Name, Home>,
    socket_imls: HashMap<Name, Box<dyn SmartSocket>>,
    thermometer_imls: HashMap<Name, Box<dyn SmartThermometer>>,
}

impl SmartHub {
    pub fn new() -> Self {
        // let mut hub = Self::default();

        // hub.register_socket_impl("example", ExampleSocket);
        // hub.register_thermometer_impl("example", ExampleThermometer);

        // hub
        Self::default()
    }

    pub fn register_socket_impl<N, S>(&mut self, name: N, socket_impl: S) -> Result<()>
    where
        N: Into<String>,
        S: SmartSocket + Clone + 'static,
    {
        match self.socket_imls.entry(name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(Box::new(socket_impl));
            }
            Entry::Occupied(_) => return Err(Error::SocketImplAlreadyRegistered),
        }

        Ok(())
    }

    pub fn unregister_socket_impl<N>(&mut self, name: N) -> Result<()>
    where
        N: AsRef<str>,
    {
        if self.socket_imls.remove(name.as_ref()).is_none() {
            return Err(Error::SocketImplNotFound);
        }

        Ok(())
    }

    pub fn register_thermometer_impl<N, S>(&mut self, name: N, thermometer_impl: S) -> Result<()>
    where
        N: Into<String>,
        S: SmartThermometer + Clone + 'static,
    {
        match self.thermometer_imls.entry(name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(Box::new(thermometer_impl));
            }
            Entry::Occupied(_) => return Err(Error::ThermometerImplAlreadyRegistered),
        }

        Ok(())
    }

    pub fn unregister_thermometer_impl<N>(&mut self, name: N) -> Result<()>
    where
        N: AsRef<str>,
    {
        if self.socket_imls.remove(name.as_ref()).is_none() {
            return Err(Error::ThermometerImplNotFound);
        }

        Ok(())
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

    pub fn iter(&self) -> impl Iterator<Item = &Home> {
        self.home_list.values()
    }
}
