use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeKey {
    pub id: String,
    pub timestamp: u64,
}

pub struct MapManager {
    pub map: BTreeMap<CompositeKey, u64>,
}

impl MapManager {
    fn init() -> Self {
        let mut map: BTreeMap<CompositeKey, u64> = BTreeMap::new();

        MapManager { map }
    }
}
