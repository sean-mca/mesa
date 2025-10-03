mod actix_server;
use register::RegisterRequest;
use register::register_client::RegisterClient;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};
use tracing::info;

pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
async fn launch_servers() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let mut client =
        RegisterClient::connect("http://mesa-coordinator-service.default.svc.cluster.local:50051")
            .await?;
    info!("service init, connected to coordinator:50051 OK");
    let pod_ip =
        std::env::var("POD_IP").expect("unable to get podIP, cannot register with coordinator");

    info!("service init, pod IP acquired OK: {}", &pod_ip);
    let actix_future = actix_server::create_actix_server();
    let heartbeat_interval = std::env::var("HEARTBEAT_INTERVAL").unwrap_or(3);
    let heartbeat = {
        tokio::spawn(async move {
            loop {
                let now = SystemTime::now();
                let duration_since_epoch =
                    now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                let timestamp_secs = duration_since_epoch.as_secs();

                let req = tonic::Request::new(RegisterRequest {
                    timestamp: timestamp_secs,
                    ip: pod_ip.to_string(),
                });

                let res = client.reg(req).await.expect("utoh");
                info!("{:?}", res);
                sleep(Duration::from_secs(heartbeat_interval)).await;
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
