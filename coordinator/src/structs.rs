use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CompositeKey {
    pub id: String,
    pub timestamp: u64,
}
