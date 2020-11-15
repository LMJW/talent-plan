//! txlog defines the transaction log of the key value pair store.
//!
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub logid: usize,
    pub operation: LogOperation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogPointer {
    pub offset: usize,
    pub size: usize,
}

impl LogPointer {
    pub fn new() -> Self {
        Self { offset: 0, size: 0 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LogOperation {
    Set(String, String),
    Rm(String),
}
