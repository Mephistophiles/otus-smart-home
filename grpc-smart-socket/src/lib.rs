use async_trait::async_trait;
use smart_home_lib::{SmartDevice, SmartSocket};
use smart_socket::{socket_client::SocketClient, CurrentPowerRequest, OffRequest, OnRequest};
use tokio::sync::Mutex;
use tonic::transport::Channel;

/// GRPC Socket implementation
pub struct GrpcSmartSocket {
    name: String,
    description: String,
    client: Mutex<SocketClient<Channel>>,
}

mod smart_socket {
    tonic::include_proto!("smart_home_socket");
}

impl GrpcSmartSocket {
    /// connect to GRPC socket server
    pub async fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        server_addr: String,
    ) -> Self {
        let client = SocketClient::connect(server_addr).await.expect("connect");
        Self {
            name: name.into(),
            description: description.into(),
            client: Mutex::new(client),
        }
    }
}

impl SmartDevice for GrpcSmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[async_trait]
impl SmartSocket for GrpcSmartSocket {
    async fn on(&self) -> smart_home_lib::error::Result<()> {
        self.client
            .lock()
            .await
            .on(OnRequest {})
            .await
            .map_err(|_| smart_home_lib::error::Error::InternalError)?;
        Ok(())
    }

    async fn off(&self) -> smart_home_lib::error::Result<()> {
        self.client
            .lock()
            .await
            .off(OffRequest {})
            .await
            .map_err(|_| smart_home_lib::error::Error::InternalError)?;
        Ok(())
    }

    async fn current_power(&self) -> smart_home_lib::error::Result<f64> {
        let response = self
            .client
            .lock()
            .await
            .current_power(CurrentPowerRequest {})
            .await
            .map_err(|_| smart_home_lib::error::Error::InternalError)?;

        Ok(response.into_inner().current_power)
    }
}
