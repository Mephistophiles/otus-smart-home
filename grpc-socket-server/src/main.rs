use std::{net::SocketAddr, sync::Arc};

use rand::Rng;
use socket::{
    socket_server::{Socket, SocketServer},
    CurrentPowerRequest, CurrentPowerResponse, OffRequest, OffResponse, OnRequest, OnResponse,
};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

mod socket {
    tonic::include_proto!("smart_home_socket");
}

struct State {
    state: bool,
    current_power: f64,
}

struct MySocket {
    state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl Socket for MySocket {
    async fn on(
        &self,
        _request: Request<OnRequest>,
    ) -> std::result::Result<Response<OnResponse>, Status> {
        let reply = OnResponse {};
        let mut state = self.state.lock().await;
        let mut rng = rand::thread_rng();
        state.state = true;
        state.current_power = rng.gen_range(1.0..=200.0);

        log::info!("Handle ON command, generate {} power", state.current_power);

        Ok(Response::new(reply))
    }

    async fn off(
        &self,
        _request: Request<OffRequest>,
    ) -> std::result::Result<Response<OffResponse>, Status> {
        let reply = OffResponse {};
        let mut state = self.state.lock().await;
        state.state = false;
        state.current_power = 0.0;

        log::info!("Handle OFF command");

        Ok(Response::new(reply))
    }

    async fn current_power(
        &self,
        _request: Request<CurrentPowerRequest>,
    ) -> std::result::Result<Response<CurrentPowerResponse>, Status> {
        let state = self.state.lock().await;
        let reply = CurrentPowerResponse {
            current_power: state.current_power,
        };

        log::info!("Return {} power", reply.current_power);

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    let socket = MySocket {
        state: Arc::new(Mutex::new(State {
            state: false,
            current_power: 0.0,
        })),
    };

    Server::builder()
        .add_service(SocketServer::new(socket))
        .serve(addr)
        .await?;

    Ok(())
}
