use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct CompositeKey {
    pub ip: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MapManager {
    pub map: BTreeMap<CompositeKey, u64>,
}

impl MapManager {
    pub fn init() -> Self {
        let mut map = BTreeMap::new();

        MapManager { map }
    }

    pub async fn clean(&mut self) {
        let now = SystemTime::now();
        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let timestamp_secs = duration_since_epoch.as_secs() - 30;

        let _ = &self
            .map
            .retain(|key, _value| key.timestamp > timestamp_secs);
    }

    pub async fn listen_for_changes(&mut self, mut receiver: UnboundedReceiver<CompositeKey>) {
        while let Some(msg) = receiver.recv().await {
            let message_data = CompositeKey {
                ip: msg.ip,
                timestamp: msg.timestamp,
            };

            let _ = &self.map.insert(message_data, msg.timestamp);
        }
    }

    pub async fn begin_listening_and_cleaning(
        &mut self,
        mut receiver: UnboundedReceiver<CompositeKey>,
    ) {
        &self.listen_for_changes(receiver);
        let _ = &self.clean();
    }
}
