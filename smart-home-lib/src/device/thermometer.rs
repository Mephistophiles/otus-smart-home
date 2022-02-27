use async_trait::async_trait;

use crate::{error::Result, Device, SmartDevice};

/// Smart thermometer (get themperature)
#[async_trait]
pub trait SmartThermometer: SmartDevice + Send {
    /// Get current temperature
    async fn current_temperature(&self) -> Result<f64>;
}

impl From<Box<dyn SmartThermometer>> for Device {
    fn from(thermometer: Box<dyn SmartThermometer>) -> Self {
        Device::Thermometer(thermometer)
    }
}
