use derivative::Derivative;

use self::udp_smart_thermometer::UdpSmartThermometer;
use crate::{error::Result, SmartDevice};

mod udp_smart_thermometer;

/// Smart thermometer (get themperature)
#[derive(Derivative)]
#[derivative(Debug)]
pub struct SmartThermometer {
    name: String,
    description: String,
    #[derivative(Debug = "ignore")]
    thermometer: UdpSmartThermometer,
}

impl SmartThermometer {
    pub async fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        server_addr: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            thermometer: UdpSmartThermometer::new(server_addr.into()).await,
        }
    }

    /// Get current temperature
    pub async fn current_temperature(&self) -> Result<f64> {
        self.thermometer.current_temperature().await
    }
}

impl SmartDevice for SmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn device_type(&self) -> &str {
        "thermometer"
    }
}
