use std::sync::Arc;

use derivative::Derivative;
use iced::{
    alignment, button,
    scrollable::{self, Scrollable},
    text_input::{self, TextInput},
    Application, Button, Column, Command, Container, Element, Length, Row, Settings, Text,
};

use self::{
    device::{DeviceMessage, SocketDeviceView, ThermoDeviceView},
    home::{HomeMessage, HomeView},
    room::{RoomMessage, RoomView},
};

mod api;
mod device;
mod fonts;
mod home;
mod room;
mod style;

pub fn main() -> iced::Result {
    Home::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Vec<HomeView>),
    Back,
    HomeAdd,
    HomeAdded(HomeView),
    HomeDeleted(String),
    HomeChange(Arc<String>, HomeMessage),
    RoomView((Arc<String>, Vec<RoomView>)),
    RoomAdd,
    RoomAdded(RoomView),
    RoomDeleted(String),
    RoomChange(Arc<String>, RoomMessage),

    DeviceView((Arc<String>, (Vec<ThermoDeviceView>, Vec<SocketDeviceView>))),

    ThermometerChange(Arc<String>, DeviceMessage),
    SocketChange(Arc<String>, DeviceMessage),

    ThermometerAdd,
    ThermometerAdded(ThermoDeviceView),
    SocketAdd,
    SocketToggled(String),
    SocketAdded(SocketDeviceView),
    ThermometerDeleted(String),
    SocketDeleted(String),

    InputChanged(String),
    ThermoInputChanged(String),
    SocketInputChanged(String),
}

#[derive(Derivative)]
#[derivative(Default, Debug, Clone)]
enum CurrentView {
    #[derivative(Default)]
    Home,
    Room,
    Device,
}

#[derive(Debug, Default, Clone)]
struct State {
    scroll: scrollable::State,

    current_view: CurrentView,

    input: text_input::State,
    input_value: String,

    back_button: button::State,

    home_name: Option<Arc<String>>,
    home_list: Vec<HomeView>,

    room_name: Option<Arc<String>>,
    room_list: Vec<RoomView>,

    thermo_input: text_input::State,
    thermo_input_value: String,
    thermometer_list: Vec<ThermoDeviceView>,

    socket_input: text_input::State,
    socket_input_value: String,
    socket_list: Vec<SocketDeviceView>,
}

enum Home {
    Loading,
    Loaded(Box<State>),
}

impl Application for Home {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Home, Command<Message>) {
        (
            Home::Loading,
            Command::perform(
                async {
                    api::home::add("one").await; // FIXME: remove this
                    api::home::add("two").await; // FIXME: remove this
                    api::home::get_list().await
                },
                Message::Loaded,
            ),
        )
    }

    fn title(&self) -> String {
        match self {
            Home::Loading => "Loading...".to_string(),
            Home::Loaded(_) => "Smart Hub".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Home::Loading = self {
            match message {
                Message::Loaded(home_list) => {
                    *self = Home::Loaded(Box::new(State {
                        home_list,
                        ..Default::default()
                    }));
                }
                x => unreachable!("{x:?}"),
            }

            return Command::none();
        }

        let state = match self {
            Home::Loading => unreachable!(),
            Home::Loaded(state) => state,
        };

        match message {
            Message::Loaded(_) => unreachable!(),
            Message::InputChanged(input) => {
                state.input_value = input;
            }
            Message::ThermoInputChanged(input) => {
                state.thermo_input_value = input;
            }
            Message::SocketInputChanged(input) => {
                state.socket_input_value = input;
            }
            Message::HomeAdd => {
                if !state.input_value.is_empty() {
                    let input_value = std::mem::take(&mut state.input_value);
                    return Command::perform(
                        async move { api::home::add(&input_value).await },
                        Message::HomeAdded,
                    );
                }
            }
            Message::HomeAdded(home) => state.home_list.push(home),
            Message::HomeChange(name, submessage) => match submessage {
                HomeMessage::Delete => {
                    return Command::perform(
                        async move { api::home::delete(&name).await },
                        Message::HomeDeleted,
                    )
                }
                HomeMessage::Edit => {
                    return Command::perform(
                        async move { (name.clone(), api::room::get_list(&name).await) },
                        Message::RoomView,
                    );
                }
            },
            Message::HomeDeleted(home) => state.home_list.retain(|h| h.name() != home),
            Message::RoomView((name, room_list)) => {
                state.current_view = CurrentView::Room;
                state.home_name = Some(name);
                state.room_list = room_list;
            }
            Message::RoomAdd => {
                if !state.input_value.is_empty() {
                    let input_value = std::mem::take(&mut state.input_value);
                    let home_name = state.home_name.as_ref().unwrap().clone();
                    return Command::perform(
                        async move { api::room::add(&home_name, &input_value).await },
                        Message::RoomAdded,
                    );
                }
            }
            Message::RoomAdded(room) => state.room_list.push(room),
            Message::RoomChange(room, submessage) => match submessage {
                RoomMessage::Edit => {
                    let home = state.home_name.as_ref().unwrap().clone();

                    return Command::perform(
                        async move {
                            (
                                room.clone(),
                                api::device::get_device_list(&home, &room).await,
                            )
                        },
                        Message::DeviceView,
                    );
                }
                RoomMessage::Delete => {
                    let home = state.home_name.as_ref().unwrap().clone();
                    return Command::perform(
                        async move { api::room::delete(&home, &room).await },
                        Message::RoomDeleted,
                    );
                }
            },
            Message::RoomDeleted(room) => state.room_list.retain(|r| r.name() != room),
            Message::ThermometerChange(name, submessage) => {
                let home = state.home_name.as_ref().unwrap().clone();
                let room = state.room_name.as_ref().unwrap().clone();

                match submessage {
                    DeviceMessage::Delete => {
                        return Command::perform(
                            async move { api::device::delete_thermometer(&home, &room, &name).await },
                            Message::ThermometerDeleted,
                        );
                    }
                    DeviceMessage::Toggle => {
                        let device = state
                            .socket_list
                            .iter_mut()
                            .find(|s| s.name() == name.as_ref())
                            .unwrap();
                        device.state(!device.get_state());
                    }
                }
            }
            Message::SocketChange(name, submessage) => {
                let home = state.home_name.as_ref().unwrap().clone();
                let room = state.room_name.as_ref().unwrap().clone();
                let current_state = state
                    .socket_list
                    .iter()
                    .find(|socket| socket.name() == name.as_ref())
                    .map(|socket| socket.get_state())
                    .unwrap_or_default();

                match submessage {
                    DeviceMessage::Delete => {
                        return Command::perform(
                            async move { api::device::delete_socket(&home, &room, &name).await },
                            Message::SocketDeleted,
                        );
                    }
                    DeviceMessage::Toggle => {
                        return Command::perform(
                            async move {
                                api::device::toggle_socket(&home, &room, &name, !current_state)
                                    .await
                            },
                            Message::SocketToggled,
                        );
                    }
                }
            }
            Message::DeviceView((name, (thermo, sockets))) => {
                state.room_name = Some(name);
                state.current_view = CurrentView::Device;
                state.thermometer_list = thermo;
                state.socket_list = sockets;
            }
            Message::ThermometerAdd => {
                if !state.thermo_input_value.is_empty() {
                    let home = state.home_name.as_ref().unwrap().clone();
                    let room = state.room_name.as_ref().unwrap().clone();
                    let tokens = std::mem::take(&mut state.thermo_input_value);
                    let (device_name, server) = match tokens.rsplit_once(' ') {
                        Some(pair) => pair,
                        None => return Command::none(),
                    };

                    let device_name = device_name.to_string();
                    let server = server.to_string();

                    return Command::perform(
                        async move {
                            api::device::add_thermometer(&home, &room, &device_name, &server).await
                        },
                        Message::ThermometerAdded,
                    );
                }
            }
            Message::ThermometerAdded(thermo) => state.thermometer_list.push(thermo),
            Message::ThermometerDeleted(thermo) => {
                state.thermometer_list.retain(|t| t.name() != thermo)
            }
            Message::SocketAdd => {
                if !state.socket_input_value.is_empty() {
                    let home = state.home_name.as_ref().unwrap().clone();
                    let room = state.room_name.as_ref().unwrap().clone();
                    let tokens = std::mem::take(&mut state.socket_input_value);
                    let (device_name, server) = match tokens.rsplit_once(' ') {
                        Some(pair) => pair,
                        None => return Command::none(),
                    };

                    let device_name = device_name.to_string();
                    let server = server.to_string();

                    return Command::perform(
                        async move {
                            api::device::add_socket(&home, &room, &device_name, &server).await
                        },
                        Message::SocketAdded,
                    );
                }
            }
            Message::SocketAdded(socket) => state.socket_list.push(socket),
            Message::SocketDeleted(socket) => state.socket_list.retain(|t| t.name() != socket),
            Message::SocketToggled(socket) => println!("{:?}", socket),
            Message::Back => {
                state.current_view = match state.current_view {
                    CurrentView::Device => CurrentView::Room,
                    CurrentView::Room => CurrentView::Home,
                    CurrentView::Home => unreachable!(),
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Home::Loading => Self::render_loading(),
            Home::Loaded(ctx) => Self::render(ctx),
        }
    }
}

impl Home {
    fn render_loading<'a>() -> Element<'a, Message> {
        Container::new(
            Text::new("Loading...")
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
    }

    fn render_home(state: &mut State) -> Element<Message> {
        let title = Text::new("SmartHub")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let input = TextInput::new(
            &mut state.input,
            "Enter home name",
            &state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::HomeAdd);

        let home: Element<_> = state
            .home_list
            .iter_mut()
            .map(|h| (Arc::new(h.name().to_string()), h))
            .fold(Column::new().spacing(20), |column, (name, home)| {
                column.push(
                    home.view()
                        .map(move |message| Message::HomeChange(name.clone(), message)),
                )
            })
            .into();

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(input)
            .push(home);

        Scrollable::new(&mut state.scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }

    fn render_room(state: &mut State) -> Element<Message> {
        let title = Text::new(format!("Home: {}", state.home_name.as_ref().unwrap()))
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let input = TextInput::new(
            &mut state.input,
            "Enter room name",
            &state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::RoomAdd);

        let home: Element<_> = state
            .room_list
            .iter_mut()
            .map(|h| (Arc::new(h.name().to_string()), h))
            .fold(Column::new().spacing(20), |column, (name, home)| {
                column.push(
                    home.view()
                        .map(move |message| Message::RoomChange(name.clone(), message)),
                )
            })
            .into();

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(
                Button::new(&mut state.back_button, fonts::back_button())
                    .on_press(Message::Back)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .push(input)
            .push(home);

        Scrollable::new(&mut state.scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }

    fn render_device(state: &mut State) -> Element<Message> {
        let title = Text::new(format!("Room: {}", state.room_name.as_ref().unwrap()))
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let input_thermometer = TextInput::new(
            &mut state.thermo_input,
            "Enter thermometer name <space> server",
            &state.thermo_input_value,
            Message::ThermoInputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::ThermometerAdd);

        let input_socket = TextInput::new(
            &mut state.socket_input,
            "Enter socket name <space> server",
            &state.socket_input_value,
            Message::SocketInputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::SocketAdd);

        let thermo: Element<_> = state
            .thermometer_list
            .iter_mut()
            .map(|h| (Arc::new(h.name().to_string()), h))
            .fold(
                Row::new().width(Length::Fill).spacing(20),
                |column, (name, home)| {
                    column.push(
                        home.view()
                            .map(move |message| Message::ThermometerChange(name.clone(), message)),
                    )
                },
            )
            .into();

        let socket: Element<_> = state
            .socket_list
            .iter_mut()
            .map(|h| (Arc::new(h.name().to_string()), h))
            .fold(
                Row::new().width(Length::Fill).spacing(20),
                |column, (name, home)| {
                    column.push(
                        home.view()
                            .map(move |message| Message::SocketChange(name.clone(), message)),
                    )
                },
            )
            .into();

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(
                Button::new(&mut state.back_button, fonts::back_button())
                    .on_press(Message::Back)
                    .padding(10)
                    .style(style::Button::Icon),
            )
            .push(Row::new().push(input_thermometer).push(input_socket))
            .push(Row::new().push(thermo).push(socket));

        Scrollable::new(&mut state.scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }

    fn render(state: &mut State) -> Element<Message> {
        match state.current_view {
            CurrentView::Home => Self::render_home(state),
            CurrentView::Room => Self::render_room(state),
            CurrentView::Device => Self::render_device(state),
        }
    }
}
