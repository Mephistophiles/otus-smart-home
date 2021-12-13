use std::collections::{hash_map::Entry, HashMap};

/// Room management
use crate::device::{Device, SmartDevice, SmartSocket, SmartThermometer};
use crate::error::{Error, Result};

/// A room in the Home
/// ```
/// use smart_home_lib::{Device, Room};
///
/// let mut room = Room::new("Room 1");
///
/// assert_eq!(room.name(), "Room 1");
/// assert_eq!(room.device_iter().count(), 0);
/// ```
#[derive(Debug, PartialEq)]
pub struct Room {
    /// Name of the room
    name: String,
    /// List of devices in the current room
    devices: HashMap<String, Device>,
}

impl Room {
    /// Construct a new empty room
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            devices: Default::default(),
        }
    }

    /// Gets room name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add device to the Room
    pub fn add_device<T>(&mut self, device: T) -> Result<()>
    where
        T: SmartDevice + Into<Device>,
    {
        match self.devices.entry(device.name().to_string()) {
            Entry::Occupied(_) => Err(Error::DeviceAlreadyExists(device.into())),
            Entry::Vacant(entry) => {
                entry.insert(device.into());
                Ok(())
            }
        }
    }

    /// Del device from the Room
    pub fn del_device(&mut self, name: &str) -> Option<Device> {
        self.devices.remove(name)
    }

    /// Get device by name
    pub fn device(&self, name: &str) -> Option<&Device> {
        self.devices.get(name)
    }

    /// Get device iterator in the current room
    pub fn device_iter(&self) -> impl Iterator<Item = &Device> {
        self.devices.iter().map(|(_, device)| device)
    }

    /// Get mutable device iterator in the current room
    pub fn device_iter_mut(&mut self) -> impl Iterator<Item = &mut Device> {
        self.devices.iter_mut().map(|(_, device)| device)
    }

    /// Get socket devices
    pub fn socket_devices(&self) -> impl Iterator<Item = &Box<dyn SmartSocket>> {
        self.device_iter().filter_map(|device| match device {
            Device::Socket(socket) => Some(socket),
            _ => None,
        })
    }

    /// Get thermometer devices
    pub fn thermometer_devices(&self) -> impl Iterator<Item = &Box<dyn SmartThermometer>> {
        self.device_iter().filter_map(|device| match device {
            Device::Thermometer(thermometer) => Some(thermometer),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::device::hardcoded_devices::{ExampleSocket, ExampleThermometer};

    fn get_predefined_thermometer() -> Box<dyn SmartThermometer> {
        ExampleThermometer::new("smart thermometer", "Handmade thermometer").into()
    }

    fn get_predefined_socket() -> Box<dyn SmartSocket> {
        ExampleSocket::new("smart socket", "Handmade socket").into()
    }

    #[test]
    fn example() {
        let mut room = Room::new("room");
        assert_eq!(room.device_iter().count(), 0);
        assert_eq!(room.device("NOT_FOUND"), None);

        room.add_device(get_predefined_thermometer()).unwrap();

        assert_eq!(room.device_iter().count(), 1);
        assert_eq!(
            room.device("smart thermometer"),
            Some(&Device::Thermometer(get_predefined_thermometer()))
        );
        assert!(room.device_iter().any(|d| d.name() == "smart thermometer"));
        assert!(room
            .device_iter()
            .any(|d| d.description() == "Handmade thermometer"));

        let smart_device = room.device_iter().find(|d| d.name() == "smart thermometer");
        let smart_device = format!("{:?}", smart_device);

        assert_eq!(smart_device, "Some(Thermometer(\"smart thermometer\"))");

        room.add_device(get_predefined_socket()).unwrap();

        assert!(matches!(
            room.add_device(get_predefined_socket()),
            Err(Error::DeviceAlreadyExists(_))
        ));

        assert_eq!(room.device_iter().count(), 2);
        assert_eq!(room.device_iter_mut().count(), 2);
        assert_eq!(
            room.device("smart socket"),
            Some(&Device::Socket(get_predefined_socket()))
        );
        let smart_device = room.device_iter().find(|d| d.name() == "smart socket");
        let smart_device = format!("{:?}", smart_device);

        assert_eq!(smart_device, "Some(Socket(\"smart socket\"))");

        assert_eq!(room.thermometer_devices().count(), 1);
        assert_eq!(room.socket_devices().count(), 1);

        let deleted_device = room.del_device("smart socket");
        assert!(deleted_device.is_some());
        let deleted_device = deleted_device.unwrap();

        assert_eq!(deleted_device.name(), "smart socket");
        assert_eq!(deleted_device.description(), "Handmade socket");
        assert!(matches!(deleted_device, Device::Socket(_)));
    }
}
