use std::sync::Arc;

use tokio::{
    net::UdpSocket,
    sync::Mutex,
    time::{self, Duration},
};

use crate::error::Result;

#[derive(Debug)]
pub struct UdpSmartThermometer {
    temperature: Arc<Mutex<f64>>,
}

impl UdpSmartThermometer {
    pub async fn new(server_addr: String) -> Self {
        let temperature = Arc::new(Mutex::new(0.0));

        {
            let temperature = temperature.clone();

            tokio::spawn(async move {
                loop {
                    let sock = if let Ok(sock) = UdpSocket::bind(&server_addr).await {
                        sock
                    } else {
                        tokio::time::sleep(Duration::from_secs(10)).await;
                        continue;
                    };

                    let sleep = time::sleep(Duration::from_secs(10));
                    tokio::pin!(sleep);

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
                }
            });
        }

        Self { temperature }
    }

    pub async fn current_temperature(&self) -> Result<f64> {
        Ok(*self.temperature.lock().await)
    }
}
