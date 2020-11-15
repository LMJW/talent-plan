//! txlog defines the transaction log of the key value pair store.
//!
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
/// tuple(logid, log_type)
pub struct LogEntry(pub usize, pub LogOperation);

#[derive(Serialize, Deserialize, Debug)]
/// tuple (offset, size)
pub struct LogPointer(pub usize, pub usize);

impl LogPointer {
    pub fn new() -> Self {
        Self(0, 0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LogOperation {
    Set(String, String),
    Rm(String),
}
