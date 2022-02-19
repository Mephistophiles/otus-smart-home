use smart_socket::{socket_client::SocketClient, CurrentPowerRequest, OffRequest, OnRequest};

use crate::error::{Error, Result};

/// GRPC Socket implementation
#[derive(Debug)]
pub struct GrpcSmartSocket {
    server_addr: tonic::transport::Uri,
}

mod smart_socket {
    tonic::include_proto!("smart_home_socket");
}

impl GrpcSmartSocket {
    /// connect to GRPC socket server
    pub async fn new(server_addr: String) -> Self {
        Self {
            server_addr: server_addr.try_into().expect("valid server address"),
        }
    }

    pub async fn on(&self) -> Result<()> {
        SocketClient::connect(self.server_addr.clone())
            .await
            .expect("connect")
            .on(OnRequest {})
            .await
            .map_err(|_| Error::Internal)?;
        Ok(())
    }

    pub async fn off(&self) -> Result<()> {
        SocketClient::connect(self.server_addr.clone())
            .await
            .expect("connect")
            .off(OffRequest {})
            .await
            .map_err(|_| Error::Internal)?;
        Ok(())
    }

    pub async fn current_power(&self) -> Result<f64> {
        let response = SocketClient::connect(self.server_addr.clone())
            .await
            .expect("connect")
            .current_power(CurrentPowerRequest {})
            .await
            .map_err(|_| Error::Internal)?;

        Ok(response.into_inner().current_power)
    }
}
