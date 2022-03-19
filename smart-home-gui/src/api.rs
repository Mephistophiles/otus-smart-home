use itertools::Itertools;

const API_SERVER: &str = "http://127.0.0.1:4080/home";

fn make_path<const N: usize>(parts: [&str; N]) -> String {
    std::iter::once(API_SERVER)
        .chain(parts.iter().copied())
        .join("/")
}

fn make_path_lookup<const N: usize>(parts: [&str; N]) -> String {
    let mut str = std::iter::once(API_SERVER)
        .chain(parts.iter().copied())
        .join("/");

    str.push('/');
    str
}

pub mod home {
    use super::{make_path, make_path_lookup};
    use crate::HomeView;

    pub async fn get_list() -> Vec<HomeView> {
        async fn get_list_optional() -> Option<Vec<HomeView>> {
            reqwest::get(make_path_lookup([]))
                .await
                .ok()?
                .json()
                .await
                .ok()?
        }

        get_list_optional().await.unwrap_or_default()
    }

    pub async fn add(home: &str) -> HomeView {
        reqwest::Client::new()
            .put(make_path([home]))
            .send()
            .await
            .ok();

        HomeView::new(home)
    }

    pub async fn delete(home: &str) -> String {
        reqwest::Client::new()
            .delete(make_path([home]))
            .send()
            .await
            .ok();

        home.to_string()
    }
}

pub mod room {
    use super::{make_path, make_path_lookup};
    use crate::RoomView;

    pub async fn get_list(home: &str) -> Vec<RoomView> {
        async fn get_list_optional(home: &str) -> Option<Vec<RoomView>> {
            reqwest::get(make_path_lookup([home]))
                .await
                .ok()?
                .json()
                .await
                .ok()?
        }

        get_list_optional(home).await.unwrap_or_default()
    }

    pub async fn add(home: &str, room: &str) -> RoomView {
        reqwest::Client::new()
            .put(make_path([home, room]))
            .send()
            .await
            .ok();

        RoomView::new(room)
    }

    pub async fn delete(home: &str, room: &str) -> String {
        reqwest::Client::new()
            .delete(make_path([home, room]))
            .send()
            .await
            .ok();

        room.to_string()
    }
}

pub mod device {
    use serde::{Deserialize, Serialize};

    use super::{make_path, make_path_lookup};
    use crate::{SocketDeviceView, ThermoDeviceView};

    #[derive(Serialize)]
    struct WebDevice {
        name: String,
        description: String,
        server_addr: String,
    }

    #[derive(Deserialize)]
    struct Device {
        name: String,
        device_type: String,
    }

    pub async fn get_device_list(
        home: &str,
        room: &str,
    ) -> (Vec<ThermoDeviceView>, Vec<SocketDeviceView>) {
        async fn get_list_optional(
            home: &str,
            room: &str,
        ) -> Option<(Vec<ThermoDeviceView>, Vec<SocketDeviceView>)> {
            let device_list = reqwest::get(make_path_lookup([home, room]))
                .await
                .ok()?
                .json::<Vec<Device>>()
                .await
                .ok()?;

            let mut thermo_list = vec![];
            let mut socket_list = vec![];

            for device in device_list {
                if device.device_type == "thermometer" {
                    thermo_list.push(ThermoDeviceView::new(device.name));
                } else {
                    socket_list.push(SocketDeviceView::new(device.name));
                }
            }

            Some((thermo_list, socket_list))
        }

        get_list_optional(home, room).await.unwrap_or_default()
    }

    pub async fn add_thermometer(
        home: &str,
        room: &str,
        device: &str,
        server: &str,
    ) -> ThermoDeviceView {
        reqwest::Client::new()
            .put(make_path([home, room, "thermometer"]))
            .json(&WebDevice {
                name: device.to_string(),
                description: "TODO".to_string(),
                server_addr: server.to_string(),
            })
            .send()
            .await
            .ok();

        ThermoDeviceView::new(device)
    }

    pub async fn delete_thermometer(home: &str, room: &str, device: &str) -> String {
        reqwest::Client::new()
            .delete(make_path([home, room, device]))
            .send()
            .await
            .ok();

        device.to_string()
    }

    pub async fn add_socket(
        home: &str,
        room: &str,
        device: &str,
        server: &str,
    ) -> SocketDeviceView {
        reqwest::Client::new()
            .put(make_path([home, room, "socket"]))
            .json(&WebDevice {
                name: device.to_string(),
                description: "TODO".to_string(),
                server_addr: server.to_string(),
            })
            .send()
            .await
            .ok();

        SocketDeviceView::new(device)
    }

    pub async fn delete_socket(home: &str, room: &str, device: &str) -> String {
        reqwest::Client::new()
            .delete(make_path([home, room, device]))
            .send()
            .await
            .ok();

        device.to_string()
    }

    pub async fn toggle_socket(home: &str, room: &str, device: &str, needed_state: bool) -> String {
        reqwest::Client::new()
            .post(make_path([
                home,
                room,
                device,
                if needed_state { "on" } else { "off" },
            ]))
            .send()
            .await
            .ok();

        device.to_string()
    }
}
