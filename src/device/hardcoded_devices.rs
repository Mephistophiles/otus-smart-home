#![allow(dead_code)]
/// only for examples
use std::cell::Cell;

use crate::{SmartDevice, SmartPlug, SmartThermometer};

/// Thermometer for the tests
pub(crate) struct ExampleThermometer {
    current_temperature: Cell<f64>,
    name: String,
    description: String,
}

/// Thermometer for the tests
pub(crate) struct ExamplePlug {
    current_state: Cell<bool>,
    current_power: Cell<f64>,
    name: String,
    description: String,
}

impl From<ExampleThermometer> for Box<dyn SmartThermometer> {
    fn from(thermometer: ExampleThermometer) -> Self {
        Box::new(thermometer)
    }
}

impl ExampleThermometer {
    pub(crate) fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            current_temperature: Default::default(),
        }
    }

    pub(crate) fn set_current_temperature(&self, current_temperature: f64) {
        self.current_temperature.set(current_temperature)
    }
}

impl From<ExamplePlug> for Box<dyn SmartPlug> {
    fn from(plug: ExamplePlug) -> Self {
        Box::new(plug)
    }
}

impl ExamplePlug {
    pub(crate) fn new<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            current_state: Default::default(),
            current_power: Default::default(),
        }
    }

    pub(crate) fn set_current_power(&self, current_power: f64) {
        self.current_power.set(current_power)
    }

    pub(crate) fn get_current_state(&self) -> bool {
        self.current_state.get()
    }
}

impl SmartDevice for ExampleThermometer {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl SmartThermometer for ExampleThermometer {
    fn current_temperature(&self) -> crate::error::Result<f64> {
        Ok(self.current_temperature.get())
    }
}

impl SmartDevice for ExamplePlug {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl SmartPlug for ExamplePlug {
    fn on(&self) -> crate::error::Result<()> {
        self.current_state.set(true);
        Ok(())
    }

    fn off(&self) -> crate::error::Result<()> {
        self.current_state.set(false);
        Ok(())
    }

    fn current_power(&self) -> crate::error::Result<f64> {
        Ok(self.current_power.get())
    }
}
