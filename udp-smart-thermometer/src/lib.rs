use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use async_trait::async_trait;
use smart_home_lib::{SmartDevice, SmartThermometer};
use tokio::{
    net::UdpSocket,
    sync::Mutex,
    task::JoinHandle,
    time::{self, Duration},
};

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

            tokio::spawn(async move {
                let sock = UdpSocket::bind(LISTEN_ADDR).await.expect("a new socket");
                let sleep = time::sleep(Duration::from_secs(10));
                tokio::pin!(sleep);

                while (&*running).load(Ordering::Acquire) {
                    let mut buf = [0u8; 8];

                    tokio::select! {
                        maybe_bytes = sock.recv_from(&mut buf) => {
                            if let Ok((8, _)) = maybe_bytes {
                                *temperature.lock().await = f64::from_be_bytes(buf);
                            }
                        }
                        _ = &mut sleep => {
                        }
                    }

                    time::sleep(Duration::from_secs(10)).await;
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
        Ok(*self.temperature.lock().await)
    }
}

impl Drop for UdpSmartThermometer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Release);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let handles = self.join_handle.take().unwrap();
        rt.block_on(async move {
            handles.await.ok();
        });
    }
}
