use anyhow::{bail, Result};
mod hash_stuff;

/// TODO: maybe replace immutable iterator with mutable? or use interior mutability?

/// Smart thermometer (get themperature)
#[derive(Debug)]
pub struct Thermometer {}

/// Smart plug (on/off power, get current using power)
#[derive(Debug)]
pub struct Plug {}

/// Type of smart device
#[derive(Debug)]
pub enum Type {
    /// smart thermometer
    Thermometer(Thermometer),
    /// smart plug
    Plug(Plug),
}

/// Smart device
#[derive(Debug)]
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
        bail!("Not implemented")
    }
    /// Disable smart plug
    pub fn off(&self) -> Result<()> {
        bail!("Not implemented")
    }
    /// Get current using power
    pub fn current_power(&self) -> Result<f64> {
        bail!("Not implemented")
    }
}

impl Thermometer {
    /// Get current temperature
    pub fn current_temperature(&self) -> Result<f64> {
        bail!("Not implemented")
    }
}

#[cfg(test)]
mod tests {
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
        assert!(plug_res.is_err());
        assert_eq!(&plug_res.unwrap_err().to_string(), "Not implemented");

        let plug_res = plug.off();
        assert!(plug_res.is_err());
        assert_eq!(&plug_res.unwrap_err().to_string(), "Not implemented");

        let plug_res = plug.current_power();
        assert!(plug_res.is_err());
        assert_eq!(&plug_res.unwrap_err().to_string(), "Not implemented");
    }

    #[test]
    fn thermometer_test() {
        let thermometer = Thermometer {};

        let thermometer_res = thermometer.current_temperature();
        assert!(thermometer_res.is_err());
        assert_eq!(&thermometer_res.unwrap_err().to_string(), "Not implemented");
    }
}
