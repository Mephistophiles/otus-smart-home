use derivative::Derivative;

use self::grpc_smart_socket::GrpcSmartSocket;
use crate::{error::Result, SmartDevice};

mod grpc_smart_socket;

/// Smart socket (on/off power, get current using power)
#[derive(Derivative)]
#[derivative(Debug)]
pub struct SmartSocket {
    name: String,
    description: String,
    #[derivative(Debug = "ignore")]
    socket: GrpcSmartSocket,
}

impl SmartDevice for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn device_type(&self) -> &str {
        "socket"
    }
}

impl SmartSocket {
    /// connect to GRPC socket server
    pub async fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        server_addr: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            socket: GrpcSmartSocket::new(server_addr.into()).await,
        }
    }

    /// Enable smart socket
    pub async fn on(&self) -> Result<()> {
        self.socket.on().await
    }
    /// Disable smart socket
    pub async fn off(&self) -> Result<()> {
        self.socket.off().await
    }

    /// Get current using power
    pub async fn current_power(&self) -> Result<f64> {
        self.socket.current_power().await
    }
}
