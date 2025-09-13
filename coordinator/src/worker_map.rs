use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeKey {
    pub ip: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MapManager {
    pub map: Arc<Mutex<BTreeMap<CompositeKey, u64>>>,
}

impl MapManager {
    pub fn init() -> Self {
        let mut map = Arc::new(Mutex::new(BTreeMap::new()));

        MapManager { map }
    }

    pub async fn clean(&self) {
        let now = SystemTime::now();
        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let timestamp_secs = duration_since_epoch.as_secs() - 30;

        let _ = &self
            .map
            .lock()
            .expect("error acquiring lock")
            .retain(|key, _value| key.timestamp > timestamp_secs);
    }
}
