pub(crate) mod hardcoded_devices;

pub mod socket;
pub mod thermometer;

pub use socket::SmartSocket;
pub use thermometer::SmartThermometer;

pub trait SmartDevice {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// Smart device
pub enum Device {
    /// smart thermometer
    Thermometer(Box<dyn SmartThermometer>),
    /// smart socket
    Socket(Box<dyn SmartSocket>),
}

/// Blanked impl for Box<dyn Smart**DeviceType**>
impl<T> SmartDevice for Box<T>
where
    T: SmartDevice + ?Sized,
{
    fn name(&self) -> &str {
        (**self).name()
    }

    fn description(&self) -> &str {
        (**self).description()
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thermometer(thermometer) => f
                .debug_tuple("Thermometer")
                .field(&thermometer.name())
                .finish(),
            Self::Socket(socket) => f.debug_tuple("Socket").field(&socket.name()).finish(),
        }
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

    /// Gets device name
    pub fn name(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.name(),
            Device::Thermometer(thermometer) => thermometer.name(),
        }
    }

    /// Gets device description
    pub fn description(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.description(),
            Device::Thermometer(thermometer) => thermometer.description(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::device::hardcoded_devices::{ExampleSocket, ExampleThermometer};

    #[test]
    fn device_stuff() {
        let smart_socket: Box<dyn SmartSocket> =
            Box::new(ExampleSocket::new("socket", "socket in the bedroom"));
        let smart_thermometer: Box<dyn SmartThermometer> = Box::new(ExampleThermometer::new(
            "thermometer",
            "thermometer in the bedroom",
        ));

        let device = Device::new(smart_socket);
        assert_eq!(device.name(), "socket");
        assert_eq!(device.description(), "socket in the bedroom");
        assert!(matches!(&device, &Device::Socket { .. }));

        let device = Device::new(smart_thermometer);
        assert_eq!(device.name(), "thermometer");
        assert_eq!(device.description(), "thermometer in the bedroom");
        assert!(matches!(&device, &Device::Thermometer { .. }));
    }

    #[tokio::test]
    async fn socket_test() {
        let socket = ExampleSocket::new("socket", "socket in the bedroom");
        let sample_power = 100.;

        let socket_res = socket.on().await;
        assert!(matches!(socket_res, Ok(())));
        assert!(socket.get_current_state());

        let socket_res = socket.off().await;
        assert!(matches!(socket_res, Ok(())));
        assert!(!socket.get_current_state());

        socket.set_current_power(sample_power);

        let socket_res = socket.current_power().await.unwrap();
        assert_eq!(socket_res, sample_power);
    }

    #[tokio::test]
    async fn thermometer_test() {
        let thermometer = ExampleThermometer::new("thermometer", "thermometer");
        let sample_themperature = 20.;

        thermometer.set_current_temperature(sample_themperature);

        let thermometer_res = thermometer.current_temperature().await.unwrap();
        assert_eq!(thermometer_res, sample_themperature);
    }
}
