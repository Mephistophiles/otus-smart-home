use iced::{button, Alignment, Button, Element, Row, Text};
use serde::Deserialize;

use crate::{fonts, style};

#[derive(Debug, Clone)]
pub enum RoomMessage {
    Edit,
    Delete,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RoomView {
    name: String,
    #[serde(skip)]
    edit_button: button::State,
    #[serde(skip)]
    delete_button: button::State,
}

impl RoomView {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn view(&mut self) -> Element<RoomMessage> {
        Row::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new(&self.name).width(iced::Length::Fill))
            .push(
                Button::new(&mut self.edit_button, fonts::edit_icon())
                    .on_press(RoomMessage::Edit)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .push(
                Button::new(&mut self.delete_button, fonts::delete_icon())
                    .on_press(RoomMessage::Delete)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .into()
    }
}
