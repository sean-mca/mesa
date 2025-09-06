use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CompositeKey {
    pub ip: String,
    pub timestamp: u64,
}
