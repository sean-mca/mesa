use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CompositeKey {
    pub id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MapManager {
    pub map: BTreeMap<CompositeKey, u64>,
}

impl MapManager {
    pub fn init() -> Self {
        let mut map: BTreeMap<CompositeKey, u64> = BTreeMap::new();

        MapManager { map }
    }
}
