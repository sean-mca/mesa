use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use register::{RegisterRequest, RegisterResponse, register_server::Register};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use tonic::{Request, Response, Status, transport::Server};
mod worker_map;
use crate::register::register_server::RegisterServer;

mod structs;

pub mod register {
    tonic::include_proto!("register");
}

#[derive(Debug, Default)]
pub struct MyRegister {}

#[tonic::async_trait]
impl Register for MyRegister {
    async fn reg(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let reply = RegisterResponse {
            confirmation: "confirmed".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[tokio::main]
async fn launch_servers() -> Result<(), Box<dyn std::error::Error>> {
    let actix_future = {
        HttpServer::new(|| App::new().service(health))
            .bind("0.0.0.0:3000")
            .expect("error launching actix server")
            .run()
    };

    let tonic_future = {
        let addr = "[::1]:50051".parse().expect("error parsing tonic addr");
        let register = MyRegister::default();
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
