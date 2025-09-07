mod actix_server;
use register::RegisterRequest;
use register::register_client::RegisterClient;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
async fn launch_servers() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        RegisterClient::connect("http://mesa-coordinator-service.default.svc.cluster.local:50051")
            .await?;

    println!("CONNECTED OK");
    let actix_future = actix_server::create_actix_server();

    let heartbeat = {
        tokio::spawn(async move {
            loop {
                let now = SystemTime::now();
                let duration_since_epoch =
                    now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                let timestamp_secs = duration_since_epoch.as_secs();

                let req = tonic::Request::new(RegisterRequest {
                    timestamp: timestamp_secs,
                    ip: "another test".to_string(),
                });

                let res = client.reg(req).await.expect("utoh");
                println!("{:?}", res);
                sleep(Duration::from_secs(3)).await;
            }
        })
    };

    tokio::select! {
        _ = actix_future => {}
        _ = heartbeat => {}
        _ = tokio::signal::ctrl_c()=>{
            println!("\n🛑 Shutdown signal received. Cleaning up...");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    launch_servers()
}
