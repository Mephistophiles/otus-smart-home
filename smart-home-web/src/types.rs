use serde::{Deserialize, Serialize};
use smart_home_lib::{Device, Home, Room, SmartDevice};

#[derive(Serialize, Deserialize)]
pub struct WebHome {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebRoom {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebDevice {
    pub name: String,
    pub description: String,
    pub device_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebThermometer {
    pub name: String,
    pub description: String,
    pub server_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebThermometerResult {
    pub current_temperature: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocket {
    pub name: String,
    pub description: String,
    pub server_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketResult {
    pub current_power: f64,
}

impl From<Home> for WebHome {
    fn from(h: Home) -> Self {
        WebHome {
            name: h.name().to_string(),
        }
    }
}

impl From<&Home> for WebHome {
    fn from(h: &Home) -> Self {
        WebHome {
            name: h.name().to_string(),
        }
    }
}

impl From<&mut Home> for WebHome {
    fn from(h: &mut Home) -> Self {
        WebHome {
            name: h.name().to_string(),
        }
    }
}

impl From<Room> for WebRoom {
    fn from(r: Room) -> Self {
        WebRoom {
            name: r.name().to_string(),
        }
    }
}

impl From<&Room> for WebRoom {
    fn from(r: &Room) -> Self {
        WebRoom {
            name: r.name().to_string(),
        }
    }
}

impl From<&mut Room> for WebRoom {
    fn from(r: &mut Room) -> Self {
        WebRoom {
            name: r.name().to_string(),
        }
    }
}

impl From<Device> for WebDevice {
    fn from(d: Device) -> Self {
        WebDevice {
            name: d.name().to_string(),
            description: d.description().to_string(),
            device_type: d.device_type().to_string(),
        }
    }
}

impl From<&Device> for WebDevice {
    fn from(d: &Device) -> Self {
        WebDevice {
            name: d.name().to_string(),
            description: d.description().to_string(),
            device_type: d.device_type().to_string(),
        }
    }
}

impl From<&mut Device> for WebDevice {
    fn from(d: &mut Device) -> Self {
        WebDevice {
            name: d.name().to_string(),
            description: d.description().to_string(),
            device_type: d.device_type().to_string(),
        }
    }
}
