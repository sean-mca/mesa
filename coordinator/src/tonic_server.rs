use crate::{
    register::{RegisterRequest, RegisterResponse, register_server::Register},
    worker_map::{self},
};
use tonic::{Request, Response, Status};
use tracing::info;
pub mod register {
    tonic::include_proto!("register");
}

#[derive(Debug, Default)]
pub struct MyRegisterData {
    pub map: worker_map::MapManager,
}

#[derive(Debug, Default)]
pub struct MyRegister {
    pub data: MyRegisterData,
}

#[tonic::async_trait]
impl Register for MyRegister {
    async fn reg(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let r = request.into_inner();

        info!(
            "Received registration from pod: {:#?} at {t}",
            &r.ip,
            t = &r.timestamp
        );

        let _ = &self.data.map.map.lock().unwrap().insert(r.ip, r.timestamp);

        let reply = RegisterResponse {
            confirmation: "confirmed".to_string(),
        };

        Ok(Response::new(reply))
    }
}
