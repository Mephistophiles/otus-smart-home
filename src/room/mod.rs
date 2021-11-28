use std::collections::HashSet;

/// Room management
use crate::device::{self, Device, Plug, Thermometer};
use crate::error::{Error, Result};

mod hash_stuff;

/// A room in the Home
#[derive(Debug)]
pub struct Room {
    /// Name of the room
    name: String,
    /// List of devices in the current room
    devices: HashSet<Device>,
}

impl Room {
    /// Construct a new empty room
    /// ```
    /// use otus_smart_home::Room;
    ///
    /// let room = Room::new("Test");
    /// assert_eq!(room.name(), "Test");
    /// assert_eq!(room.devices().count(), 0);
    /// ```
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
    pub fn add_device(&mut self, device: Device) -> Result<()> {
        if self.devices.contains(device.name()) {
            Err(Error::DeviceAlreadyExists(device))
        } else {
            self.devices.insert(device);

            Ok(())
        }
    }

    /// Del device from the Room
    pub fn del_device(&mut self, name: &str) -> Option<Device> {
        self.devices.take(name)
    }

    /// Get device by name
    pub fn device(&self, name: &str) -> Option<&Device> {
        self.devices.get(name)
    }

    /// Gets list of devices in current room
    pub fn devices(&self) -> impl Iterator<Item = &Device> {
        self.devices.iter()
    }

    /// Get plug devices
    pub fn plug_devices(&self) -> impl Iterator<Item = (&Device, &Plug)> {
        self.devices
            .iter()
            .filter_map(|device| match device.device_type() {
                device::Type::Plug(plug) => Some((device, plug)),
                _ => None,
            })
    }

    /// Get thermometer devices
    pub fn thermometer_devices(&self) -> impl Iterator<Item = (&Device, &Thermometer)> {
        self.devices
            .iter()
            .filter_map(|device| match device.device_type() {
                device::Type::Thermometer(thermometer) => Some((device, thermometer)),
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device;

    #[test]
    fn example() {
        let mut room = Room::new("room");
        assert_eq!(room.devices().count(), 0);
        assert_eq!(room.device("NOT_FOUND"), None);

        room.add_device(Device::new(
            "smart thermometer",
            "Handmade thermometer",
            device::Type::Thermometer(device::Thermometer {}),
        ))
        .unwrap();

        assert_eq!(room.devices().count(), 1);
        assert_eq!(
            room.device("smart thermometer"),
            Some(&Device::new(
                "smart thermometer",
                "Handmade thermometer",
                device::Type::Thermometer(device::Thermometer {}),
            ))
        );

        room.add_device(Device::new(
            "smart plug",
            "Handmade plug",
            device::Type::Plug(device::Plug {}),
        ))
        .unwrap();

        assert!(matches!(
            room.add_device(Device::new(
                "smart plug",
                "Handmade plug",
                device::Type::Plug(device::Plug {}),
            )),
            Err(Error::DeviceAlreadyExists(_))
        ));

        assert_eq!(room.devices().count(), 2);
        assert_eq!(
            room.device("smart plug"),
            Some(&Device::new(
                "smart plug",
                "Handmade plug",
                device::Type::Plug(device::Plug {}),
            ))
        );

        assert_eq!(room.thermometer_devices().count(), 1);
        assert_eq!(room.plug_devices().count(), 1);

        let deleted_device = room.del_device("smart plug");
        assert!(deleted_device.is_some());
        let deleted_device = deleted_device.unwrap();

        assert_eq!(deleted_device.name(), "smart plug");
        assert_eq!(deleted_device.description(), "Handmade plug");
        assert!(matches!(
            deleted_device.device_type(),
            device::Type::Plug(_)
        ));
    }
}
