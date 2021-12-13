#![allow(dead_code)]
/// only for examples
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use async_trait::async_trait;

use crate::{SmartDevice, SmartSocket, SmartThermometer};

/// Thermometer for the tests
pub(crate) struct ExampleThermometer {
    current_temperature: Mutex<f64>,
    name: String,
    description: String,
}

/// Thermometer for the tests
pub(crate) struct ExampleSocket {
    current_state: AtomicBool,
    current_power: Mutex<f64>,
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
        *self.current_temperature.lock().unwrap() = current_temperature;
    }
}

impl From<ExampleSocket> for Box<dyn SmartSocket> {
    fn from(socket: ExampleSocket) -> Self {
        Box::new(socket)
    }
}

impl ExampleSocket {
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
        *self.current_power.lock().unwrap() = current_power;
    }

    pub(crate) fn get_current_state(&self) -> bool {
        self.current_state.load(Ordering::Relaxed)
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

#[async_trait]
impl SmartThermometer for ExampleThermometer {
    async fn current_temperature(&self) -> crate::error::Result<f64> {
        Ok(*self.current_temperature.lock().unwrap())
    }
}

impl SmartDevice for ExampleSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[async_trait]
impl SmartSocket for ExampleSocket {
    async fn on(&self) -> crate::error::Result<()> {
        self.current_state.store(true, Ordering::Relaxed);
        Ok(())
    }

    async fn off(&self) -> crate::error::Result<()> {
        self.current_state.store(false, Ordering::Relaxed);
        Ok(())
    }

    async fn current_power(&self) -> crate::error::Result<f64> {
        Ok(*self.current_power.lock().unwrap())
    }
}
