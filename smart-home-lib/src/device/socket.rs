use async_trait::async_trait;

use crate::{error::Result, Device, SmartDevice};

/// Smart socket (on/off power, get current using power)
#[async_trait]
pub trait SmartSocket: SmartDevice + Send {
    /// Enable smart socket
    async fn on(&self) -> Result<()>;
    /// Disable smart socket
    async fn off(&self) -> Result<()>;
    /// Get current using power
    async fn current_power(&self) -> Result<f64>;
}

impl From<Box<dyn SmartSocket>> for Device {
    fn from(socket: Box<dyn SmartSocket>) -> Self {
        Device::Socket(socket)
    }
}
