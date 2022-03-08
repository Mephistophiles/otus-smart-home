pub mod socket;
pub mod thermometer;

pub use socket::SmartSocket;
pub use thermometer::SmartThermometer;

pub trait SmartDevice {
    /// Gets device name
    fn name(&self) -> &str;
    /// Gets device description
    fn description(&self) -> &str;
    /// Gets device type
    fn device_type(&self) -> &str;
}

/// Smart device
#[derive(Debug)]
pub enum Device {
    /// smart thermometer
    Thermometer(SmartThermometer),
    /// smart socket
    Socket(SmartSocket),
}

impl From<SmartThermometer> for Device {
    fn from(s: SmartThermometer) -> Self {
        Device::Thermometer(s)
    }
}

impl From<SmartSocket> for Device {
    fn from(s: SmartSocket) -> Self {
        Device::Socket(s)
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Device) -> bool {
        self.name() == other.name()
    }
}

impl Device {
    /// Create a new smart device
    pub fn new<T>(device: T) -> Self
    where
        T: SmartDevice + Into<Device>,
    {
        device.into()
    }
}

impl SmartDevice for Device {
    fn name(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.name(),
            Device::Thermometer(thermometer) => thermometer.name(),
        }
    }

    fn description(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.description(),
            Device::Thermometer(thermometer) => thermometer.description(),
        }
    }

    fn device_type(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.device_type(),
            Device::Thermometer(thermometer) => thermometer.device_type(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn device_stuff() {
        let smart_socket =
            SmartSocket::new("socket", "socket in the bedroom", "https://localhost:8080").await;
        let smart_thermometer =
            SmartThermometer::new("thermometer", "thermometer in the bedroom", "0.0.0.0:81").await;

        let device = Device::new(smart_socket);
        assert_eq!(device.name(), "socket");
        assert_eq!(device.description(), "socket in the bedroom");
        assert!(matches!(&device, &Device::Socket { .. }));

        let device = Device::new(smart_thermometer);
        assert_eq!(device.name(), "thermometer");
        assert_eq!(device.description(), "thermometer in the bedroom");
        assert!(matches!(&device, &Device::Thermometer { .. }));
    }
}
