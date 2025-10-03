use crate::worker_map::CompositeKey;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

pub enum Message {
    InsertWorker(CompositeKey),
    ClearOldWorkers,
    GetWorkers(mpsc::Sender<Vec<String>>),
}
