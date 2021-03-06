use std::{thread, time::Duration};

use smart_home_lib::{Home, Room, SmartDevice, SmartHub, SmartSocket, SmartThermometer};

#[tokio::main]
async fn main() {
    let mut hub = SmartHub::new();
    let home = hub.add_home(Home::new("sweet home")).unwrap();
    let room = Room::new("bedroom");
    let socket = SmartSocket::new("socket", "smart socket", "http://127.0.0.1:50051").await;
    let thermometer =
        SmartThermometer::new("thermometer", "smart thermometer", "0.0.0.0:10000").await;

    let room = home.add_room(room).expect("empty home");
    room.add_device(socket).expect("empty room");
    room.add_device(thermometer).expect("empty room");

    let smart_socket = room
        .socket_devices()
        .find(|s| s.name() == "socket")
        .unwrap();

    let smart_thermometer = room
        .thermometer_devices()
        .find(|s| s.name() == "thermometer")
        .unwrap();

    println!("Try to off socket...");
    smart_socket.off().await.unwrap();

    println!("Try to on socket...");
    smart_socket.on().await.unwrap();

    println!("Try to get current power...");
    println!(
        "Got {} Watts...",
        smart_socket.current_power().await.unwrap()
    );

    for _ in 0..10 {
        let temperature = smart_thermometer
            .current_temperature()
            .await
            .expect("temperature");

        println!("Current temperature: {}", temperature);

        thread::sleep(Duration::from_secs(3));
    }
}
