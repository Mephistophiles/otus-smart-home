use crate::{error::Result, Device, SmartDevice};

/// Smart thermometer (get themperature)
pub trait SmartThermometer: SmartDevice {
    /// Get current temperature
    fn current_temperature(&self) -> Result<f64>;
}

impl From<Box<dyn SmartThermometer>> for Device {
    fn from(thermometer: Box<dyn SmartThermometer>) -> Self {
        Device::Thermometer(thermometer)
    }
}
