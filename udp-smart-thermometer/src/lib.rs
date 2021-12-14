use std::{
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use async_trait::async_trait;
use smart_home_lib::{SmartDevice, SmartThermometer};

const LISTEN_ADDR: &str = "0.0.0.0:10000";

pub struct UdpSmartThermometer {
    name: String,
    description: String,
    temperature: Arc<Mutex<f64>>,
    running: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<()>>,
}

impl UdpSmartThermometer {
    pub async fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let temperature = Arc::new(Mutex::new(0.0));
        let running = Arc::new(AtomicBool::new(true));

        let join_handle = Some({
            let temperature = temperature.clone();
            let running = running.clone();

            thread::spawn(move || {
                let sock = UdpSocket::bind(LISTEN_ADDR).expect("a new socket");

                sock.set_read_timeout(Some(Duration::from_secs(10)))
                    .expect("set SO_RECV_TIMEOUT");

                while (&*running).load(Ordering::Acquire) {
                    let mut buf = [0u8; 8];

                    match sock.recv_from(&mut buf) {
                        Ok((nbytes, _)) if nbytes == buf.len() => {
                            *temperature.lock().unwrap() = f64::from_be_bytes(buf);
                        }
                        _ => (),
                    };

                    thread::sleep(Duration::from_secs(10));
                }
            })
        });

        Self {
            name: name.into(),
            description: description.into(),
            temperature,
            running,
            join_handle,
        }
    }
}

impl SmartDevice for UdpSmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[async_trait]
impl SmartThermometer for UdpSmartThermometer {
    async fn current_temperature(&self) -> smart_home_lib::error::Result<f64> {
        Ok(*self.temperature.lock().unwrap())
    }
}

impl Drop for UdpSmartThermometer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Release);

        self.join_handle
            .take()
            .unwrap()
            .join()
            .expect("Normal join");
    }
}
