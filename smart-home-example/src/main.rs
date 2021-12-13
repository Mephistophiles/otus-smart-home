use grpc_smart_socket::GrpcSmartSocket;
use smart_home_lib::{Home, Room, SmartSocket};

#[tokio::main]
async fn main() {
    let mut home = Home::new("sweet home");
    let room = Room::new("bedroom");
    let socket =
        GrpcSmartSocket::new("socket", "smart socket", "http://127.0.0.1:50051".into()).await;
    let socket: Box<dyn SmartSocket> = Box::new(socket);

    let room = home.add_room(room).expect("empty home");
    room.add_device(socket).expect("empty room");

    let smart_socket = room
        .socket_devices()
        .find(|s| s.name() == "socket")
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
}
