use std::{net::UdpSocket, thread, time::Duration};

use rand::Rng;

const ANY: &str = "0.0.0.0:0";
const BROADCAST_ADDR: &str = "255.255.255.255:10000";

fn main() {
    let socket = UdpSocket::bind(ANY).expect("bind to any");
    let mut rng = rand::thread_rng();
    let mut temperature = 0.0;

    socket.set_broadcast(true).expect("set SO_BROADCAST");

    loop {
        let temperature: f64 = if temperature == 0.0 {
            rng.gen_range(0.0..30.0)
        } else {
            // делаем плавное изменение температуры
            temperature += rng.gen_range(-0.01..0.01);
            temperature
        };

        socket
            .send_to(&temperature.to_be_bytes(), BROADCAST_ADDR)
            .expect("successful send");

        thread::sleep(Duration::from_secs(3));
    }
}
