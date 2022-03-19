use iced::{button, Alignment, Button, Element, Row, Text};
use serde::Deserialize;

use crate::{fonts, style};

#[derive(Debug, Clone)]
pub enum DeviceMessage {
    Delete,
    Toggle,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThermoDeviceView {
    name: String,
    #[serde(skip)]
    current_temperature: f64,
    #[serde(skip)]
    delete_button: button::State,
}

impl ThermoDeviceView {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn view(&mut self) -> Element<DeviceMessage> {
        Row::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .push(Text::new(&self.name).width(iced::Length::Fill))
                    .push(Text::new(format!("{} °C", self.current_temperature))),
            )
            .push(
                Button::new(&mut self.delete_button, fonts::delete_icon())
                    .on_press(DeviceMessage::Delete)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .into()
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SocketDeviceView {
    name: String,
    #[serde(skip)]
    current_power: f64,
    #[serde(skip)]
    state: bool,
    #[serde(skip)]
    toggle_button: button::State,
    #[serde(skip)]
    delete_button: button::State,
}

impl SocketDeviceView {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn state(&mut self, state: bool) {
        self.state = state
    }

    pub fn view(&mut self) -> Element<DeviceMessage> {
        Row::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .push(Text::new(&self.name).width(iced::Length::Fill))
                    .push(Text::new(format!("{} Watt", self.current_power)))
                    .push(
                        Button::new(
                            &mut self.toggle_button,
                            Text::new(if self.state { "OFF" } else { "ON" }),
                        )
                        .on_press(DeviceMessage::Toggle),
                    ),
            )
            .push(
                Button::new(&mut self.delete_button, fonts::delete_icon())
                    .on_press(DeviceMessage::Delete)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .into()
    }
}
