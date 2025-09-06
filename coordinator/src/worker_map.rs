use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
// pub struct CompositeKey {
//     pub id: String,
//     pub timestamp: u64,
// }

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
// pub struct CompositeValue {
//     pub id: String,
//     pub timestamp: u64,
// }

#[derive(Debug, Clone, Default)]
pub struct MapManager {
    pub map: Arc<Mutex<BTreeMap<String, u64>>>,
}

impl MapManager {
    pub fn init() -> Self {
        let mut map = Arc::new(Mutex::new(BTreeMap::new()));

        MapManager { map }
    }
}
