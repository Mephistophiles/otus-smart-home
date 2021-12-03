use crate::error::Result;
pub(crate) mod hardcoded_devices;

pub trait SmartDevice {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// Smart thermometer (get themperature)
pub trait SmartThermometer: SmartDevice {
    /// Get current temperature
    fn current_temperature(&self) -> Result<f64>;
}

/// Smart plug (on/off power, get current using power)
pub trait SmartPlug: SmartDevice {
    /// Enable smart plug
    fn on(&self) -> Result<()>;
    /// Disable smart plug
    fn off(&self) -> Result<()>;
    /// Get current using power
    fn current_power(&self) -> Result<f64>;
}

/// Smart device
pub enum Device {
    /// smart thermometer
    Thermometer(Box<dyn SmartThermometer>),
    /// smart plug
    Plug(Box<dyn SmartPlug>),
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
            Self::Plug(plug) => f.debug_tuple("Plug").field(&plug.name()).finish(),
        }
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Device) -> bool {
        self.name() == other.name()
    }
}

impl From<Box<dyn SmartThermometer>> for Device {
    fn from(thermometer: Box<dyn SmartThermometer>) -> Self {
        Device::Thermometer(thermometer)
    }
}

impl From<Box<dyn SmartPlug>> for Device {
    fn from(plug: Box<dyn SmartPlug>) -> Self {
        Device::Plug(plug)
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::device::hardcoded_devices::{ExamplePlug, ExampleThermometer};

    #[test]
    fn device_stuff() {
        let smart_plug: Box<dyn SmartPlug> = ExamplePlug::new("plug", "plug in bedroom").into();
        let smart_thermometer: Box<dyn SmartThermometer> =
            ExampleThermometer::new("thermometer", "thermometer in bedroom").into();

        let device = Device::new(smart_plug);
        assert_eq!(device.name(), "plug");
        assert_eq!(device.description(), "plug in bedroom");
        assert!(matches!(&device, &Device::Plug { .. }));

        let device = Device::new(smart_thermometer);
        assert_eq!(device.name(), "thermometer");
        assert_eq!(device.description(), "thermometer in bedroom");
        assert!(matches!(&device, &Device::Thermometer { .. }));
    }

    #[test]
    fn plug_test() {
        let plug = ExamplePlug::new("plug", "plug in bedroom");
        let sample_power = 100.;

        let plug_res = plug.on();
        assert!(matches!(plug_res, Ok(())));
        assert!(plug.get_current_state());

        let plug_res = plug.off();
        assert!(matches!(plug_res, Ok(())));
        assert!(!plug.get_current_state());

        plug.set_current_power(sample_power);

        let plug_res = plug.current_power().unwrap();
        assert_eq!(plug_res, sample_power);
    }

    #[test]
    fn thermometer_test() {
        let thermometer = ExampleThermometer::new("thermometer", "thermometer");
        let sample_themperature = 20.;

        thermometer.set_current_temperature(sample_themperature);

        let thermometer_res = thermometer.current_temperature().unwrap();
        assert_eq!(thermometer_res, sample_themperature);
    }
}
