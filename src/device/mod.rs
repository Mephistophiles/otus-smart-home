use crate::error::{Error, Result};

pub trait SmartDevice {
    fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// Smart thermometer (get themperature)
#[derive(Debug, PartialEq)]
pub struct Thermometer {
    name: String,
    description: String,
}

/// Smart plug (on/off power, get current using power)
#[derive(Debug, PartialEq)]
pub struct Plug {
    name: String,
    description: String,
}

/// Smart device
#[derive(Debug, PartialEq)]
pub enum Device {
    /// smart thermometer
    Thermometer(Thermometer),
    /// smart plug
    Plug(Plug),
}

impl Device {
    /// Create a new smart device
    pub fn new<T>(device: T) -> Self
    where
        T: SmartDevice + Into<Device>,
    {
        device.into()
    }

    /// Gets device name
    pub fn name(&self) -> &str {
        match self {
            Device::Plug(plug) => plug.name(),
            Device::Thermometer(thermometer) => thermometer.name(),
        }
    }

    /// Gets device description
    pub fn description(&self) -> &str {
        match self {
            Device::Plug(plug) => plug.description(),
            Device::Thermometer(thermometer) => thermometer.description(),
        }
    }
}

impl SmartDevice for Plug {
    fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl From<Plug> for Device {
    fn from(plug: Plug) -> Self {
        Device::Plug(plug)
    }
}

impl SmartDevice for Thermometer {
    fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl From<Thermometer> for Device {
    fn from(thermometer: Thermometer) -> Self {
        Device::Thermometer(thermometer)
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
        let device = Device::new(Plug::new("plug", "plug in bedroom"));
        assert_eq!(device.name(), "plug");
        assert_eq!(device.description(), "plug in bedroom");
        assert!(matches!(&device, &Device::Plug { .. }));

        let device = Device::new(Thermometer::new("thermometer", "thermometer in bedroom"));
        assert_eq!(device.name(), "thermometer");
        assert_eq!(device.description(), "thermometer in bedroom");
        assert!(matches!(&device, &Device::Thermometer { .. }));
    }

    #[test]
    fn plug_test() {
        let plug = Plug::new("plug", "plug");

        let plug_res = plug.on();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));

        let plug_res = plug.off();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));

        let plug_res = plug.current_power();
        assert!(matches!(plug_res, Err(Error::NotImplemented)));
    }

    #[test]
    fn thermometer_test() {
        let thermometer = Thermometer::new("thermometer", "thermometer");

        let thermometer_res = thermometer.current_temperature();
        assert!(matches!(thermometer_res, Err(Error::NotImplemented)));
    }
}
