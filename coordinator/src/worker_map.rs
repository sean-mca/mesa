use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tracing::info;

use crate::structs::Message;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct CompositeKey {
    pub ip: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MapManager {
    pub map: BTreeMap<String, u64>,
}

impl MapManager {
    pub fn init() -> Self {
        let mut map = BTreeMap::new();

        MapManager { map }
    }

    pub async fn listen(&mut self, mut receiver: mpsc::UnboundedReceiver<Message>) {
        let clean_interval = std::env::var("CLEAN_INTERVAL")
            .ok()
            .and_then(|i| i.parse::<u64>().ok())
            .unwrap_or(10);
        while let Some(message) = receiver.recv().await {
            match message {
                Message::GetWorkers(sender) => {
                    let keys = self.map.keys().cloned().collect();
                    let _ = sender.send(keys);
                }
                Message::ClearOldWorkers => {
                    let now = SystemTime::now();
                    let duration_since_epoch = now
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards!");
                    let timestamp_secs = duration_since_epoch.as_secs() - clean_interval;

                    let _ = &self.map.retain(|_key, value| *value > timestamp_secs);

                    info!("BTreeMap cleared")
                }
                Message::InsertWorker(key) => {
                    let _ = &self.map.insert(key.ip, key.timestamp);
                }
            }
        }
    }
}
