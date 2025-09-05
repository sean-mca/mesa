mod actix_server;
mod tonic_server;
mod worker_map;
use crate::register::register_server::RegisterServer;
use tonic::transport::Server;

mod structs;

pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
async fn launch_servers() -> Result<(), Box<dyn std::error::Error>> {
    let worker_map = worker_map::MapManager::init();

    let actix_future = actix_server::create_actix_server(worker_map);

    let tonic_future = {
        let addr = "[::1]:50051".parse().expect("error parsing tonic addr");
        let register = tonic_server::MyRegister::default();
        Server::builder()
            .add_service(RegisterServer::new(register))
            .serve(addr)
    };

    tokio::select! {
        _ = actix_future => {}
        _ = tonic_future => {}
        _ = tokio::signal::ctrl_c()=>{
            println!("\n🛑 Shutdown signal received. Cleaning up...");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    launch_servers()
}
