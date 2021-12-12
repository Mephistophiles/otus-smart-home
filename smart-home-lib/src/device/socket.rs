use crate::{error::Result, Device, SmartDevice};

/// Smart socket (on/off power, get current using power)
pub trait SmartSocket: SmartDevice {
    /// Enable smart socket
    fn on(&self) -> Result<()>;
    /// Disable smart socket
    fn off(&self) -> Result<()>;
    /// Get current using power
    fn current_power(&self) -> Result<f64>;
}

impl From<Box<dyn SmartSocket>> for Device {
    fn from(socket: Box<dyn SmartSocket>) -> Self {
        Device::Socket(socket)
    }
}
