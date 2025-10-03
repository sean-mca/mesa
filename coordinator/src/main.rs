mod actix_server;
mod tonic_server;
mod worker_map;
use std::time::Duration;

use crate::{register::register_server::RegisterServer, worker_map::CompositeKey};

use tonic::transport::Server;
mod structs;
use tracing::info;

pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
async fn launch_servers() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    // TODO: this will need to be profiled and tested at scale so we don't nuke our memory footprint
    let (send, mut receive) = tokio::sync::mpsc::unbounded_channel::<CompositeKey>();
    let mut worker_map = worker_map::MapManager::init();

    info!("service init, worker_map created OK");

    let actix_future = actix_server::create_actix_server(worker_map.clone());

    info!("service init, actix_server created OK");

    let tonic_future = {
        let addr = "0.0.0.0:50051".parse().expect("error parsing tonic addr");

        let register = tonic_server::MyRegister { sender: send };
        Server::builder()
            .add_service(RegisterServer::new(register))
            .serve(addr)
    };
    info!("service init, tonic_server created OK");

    let listener = worker_map.begin_listening_and_cleaning(receive);
    tokio::select! {
        _ = actix_future => {}
        _ = tonic_future => {}
        _ = listener => {}
        _ = tokio::signal::ctrl_c()=>{
            info!("\n🛑 Shutdown signal received. Cleaning up...");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    launch_servers()
}
