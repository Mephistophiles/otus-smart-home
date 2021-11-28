use crate::error::{Error, Result};

/// TODO: maybe replace immutable iterator with mutable? or use interior mutability?

/// Smart thermometer (get themperature)
#[derive(Debug, PartialEq)]
pub struct Thermometer {}

/// Smart plug (on/off power, get current using power)
#[derive(Debug, PartialEq)]
pub struct Plug {}

/// Type of smart device
#[derive(Debug, PartialEq)]
pub enum Type {
    /// smart thermometer
    Thermometer(Thermometer),
    /// smart plug
    Plug(Plug),
}

/// Smart device
#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    description: String,
    device_type: Type,
}

impl Device {
    /// Create a new smart device
    pub fn new<T>(name: T, description: T, device_type: Type) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            device_type,
        }
    }

    /// Gets device name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets device description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get device type
    pub fn device_type(&self) -> &Type {
        &self.device_type
    }
}

impl Plug {
    /// Enable smart plug
    pub fn on(&self) -> Result<()> {
        Err(Error::NotImplemented)
    }
    /// Disable smart plug
    pub fn off(&self) -> Result<()> {
        Err(Error::NotImplemented)
    }
    /// Get current using power
    pub fn current_power(&self) -> Result<f64> {
        Err(Error::NotImplemented)
    }
}

impl Thermometer {
    /// Get current temperature
    pub fn current_temperature(&self) -> Result<f64> {
        Err(Error::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn device_stuff() {
        // TODO: ugly init
        let device = Device::new("plug", "plug in bedroom", Type::Plug(Plug {}));
        assert_eq!(device.name(), "plug");
        assert_eq!(device.description(), "plug in bedroom");
        assert!(matches!(device.device_type(), &Type::Plug(_)));
    }

    #[test]
    fn plug_test() {
        let plug = Plug {};

        let plug_res = plug.on();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));

        let plug_res = plug.off();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));

        let plug_res = plug.current_power();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));
    }

    #[test]
    fn thermometer_test() {
        let thermometer = Thermometer {};

        let thermometer_res = thermometer.current_temperature();
        assert!(matches!(thermometer_res, Err(Error::NotImplemented)));
    }
}
