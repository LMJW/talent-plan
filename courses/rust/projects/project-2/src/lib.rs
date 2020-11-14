use serde::{Deserialize, Serialize};
use serde_cbor;
use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;

mod cfg;
mod error;
mod txlog;
use error::KvErr;
use txlog::LogPointer;

pub struct KvStore {
    index: MemoryIndex,
}

#[derive(Serialize, Deserialize, Debug)]
struct MemoryIndex {
    current_lp: LogPointer,
    index_map: HashMap<String, LogPointer>,
}

pub type Result<T> = std::result::Result<T, KvErr>;

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let midx = MemoryIndex::load(path)?;
        Ok(KvStore { index: midx })
    }

    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        self.index.save(path)
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        todo!()
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }
}

impl MemoryIndex {
    fn new() -> Self {
        Self {
            current_lp: LogPointer::new(),
            index_map: HashMap::new(),
        }
    }

    fn load(path: impl Into<PathBuf>) -> Result<Self> {
        match std::fs::File::open(path.into()) {
            Ok(fd) => {
                let midx: MemoryIndex = serde_cbor::from_reader(fd)?;
                Ok(midx)
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(MemoryIndex::new()),
                _ => Err(e.into()),
            },
        }
    }

    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        let fd = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.into())?;

        serde_cbor::to_writer(fd, self)?;
        Ok(())
    }
}
