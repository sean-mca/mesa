use std::sync::Mutex;

use crate::{
    register::{RegisterRequest, RegisterResponse, register_server::Register},
    worker_map,
};
use tonic::{Request, Response, Status};

pub mod register {
    tonic::include_proto!("register");
}

#[derive(Debug, Default)]
pub struct MyRegisterData {
    map: Mutex<worker_map::MapManager>,
}

#[derive(Debug, Default)]
pub struct MyRegister {
    data: MyRegisterData,
}

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
