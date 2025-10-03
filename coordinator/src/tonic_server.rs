use crate::{
    register::{RegisterRequest, RegisterResponse, register_server::Register},
    worker_map::CompositeKey,
};
use tokio::sync::mpsc::UnboundedSender;
use tonic::{Request, Response, Status};
use tracing::info;
pub mod register {
    tonic::include_proto!("register");
}

#[derive(Debug)]
pub struct MyRegister {
    pub sender: UnboundedSender<CompositeKey>,
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

        let _ = &self.sender.send(CompositeKey {
            ip: r.ip,
            timestamp: r.timestamp,
        });

        let reply = RegisterResponse {
            confirmation: "confirmed".to_string(),
        };

        Ok(Response::new(reply))
    }
}
