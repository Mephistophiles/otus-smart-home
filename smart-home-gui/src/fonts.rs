use iced::{alignment, Font, Length, Text};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn icon(unicode: char) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Units(20))
        .size(20)
}

pub fn edit_icon() -> Text {
    icon('\u{E800}')
}

pub fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

pub fn back_button() -> Text {
    icon('\u{E801}')
}
