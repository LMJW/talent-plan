//! txlog defines the transaction log of the key value pair store.
//!
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    logid: usize,
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogPointer {
    f_id: usize,
    l_id: usize,
}

impl LogPointer {
    pub fn new() -> Self {
        Self { f_id: 0, l_id: 0 }
    }
}
