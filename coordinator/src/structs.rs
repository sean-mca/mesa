use crate::worker_map::CompositeKey;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}


//different types of messages we can send to workermap
pub enum Message {
    InsertWorker(CompositeKey),
    ClearOldWorkers,
    GetWorkers(mpsc::Sender<Vec<String>>),
}


