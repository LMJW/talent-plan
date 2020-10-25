use std::path::PathBuf;

pub struct KvStore {
    //
}

#[derive(Debug)]
pub enum KvErr {}

pub type Result<T> = std::result::Result<T, KvErr>;

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        todo!()
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
