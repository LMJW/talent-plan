use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use serde_cbor;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::path::PathBuf;

mod error;
mod txlog;
use error::KvErr;
use txlog::{LogEntry, LogOperation, LogPointer};

#[derive(Debug)]
pub struct KvStore {
    fd: std::fs::File,
    total_log: usize,
    total_bytes: usize,
    index: HashMap<String, LogPointer>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Metadata {
    total_log: usize,
    total_bytes: usize,
}

pub type Result<T> = std::result::Result<T, KvErr>;

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let fd = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path.into())?;

        // the first 1Kb is used to store the metadata
        // the first 4 bytes are used to identify the size of metadata structure
        let mut buf = vec![0u8; 1024];
        let mut metadata = Metadata::default();

        if fd.metadata()?.len() < 1024 {
            let mut head = &mut buf[0..4];

            let md_vec = serde_cbor::to_vec(&metadata)?;
            head.write_u32::<LittleEndian>(md_vec.len() as u32)?;
            buf[4..md_vec.len() + 4].clone_from_slice(&md_vec);
            fd.write_at(&buf, 0)?;
        } else {
            fd.read_exact_at(&mut buf, 0)?;
            let mut cursor = std::io::Cursor::new(&buf[0..4]);
            let offset = (cursor.read_u32::<LittleEndian>()? + 4) as usize;
            metadata = serde_cbor::from_reader(&buf[4..offset])?;
        }
        let mut kvstore = Self {
            fd,
            total_log: metadata.total_log,
            total_bytes: metadata.total_bytes,
            index: HashMap::new(),
        };

        kvstore.replay_log()?;

        Ok(kvstore)
    }

    fn save_metadata(&self) -> Result<()> {
        let metadata = Metadata {
            total_log: self.total_log,
            total_bytes: self.total_bytes,
        };
        let md = serde_cbor::to_vec(&metadata)?;
        let mut buf = vec![0u8; 1024];
        let mut head = &mut buf[0..4];
        head.write_u32::<LittleEndian>(md.len() as u32)?;

        buf[4..md.len() + 4].clone_from_slice(&md);

        self.fd.try_clone()?.write_all_at(&buf, 0)?;
        eprint!("{:?}", metadata);
        eprint!("{:?}", self.index);
        Ok(())
    }

    fn replay_log(&mut self) -> Result<()> {
        let mut fd = self.fd.try_clone()?;
        let mut buf = [0u8; 2];
        let mut offset: usize = 1024;
        for i in 0..self.total_log {
            fd.read_exact_at(&mut buf, offset as u64)?;
            let mut cursor = std::io::Cursor::new(&buf);
            let size = cursor.read_u16::<LittleEndian>()? as usize;

            let mut data = vec![0u8; size];
            fd.read_exact_at(&mut data, offset as u64 + 2)?;
            let lp: LogEntry = serde_cbor::from_reader(&data[..])?;
            match lp.1 {
                LogOperation::Set(key, _) => {
                    self.index.insert(key, LogPointer(offset - 1024, size + 2));
                }
                LogOperation::Rm(key) => todo!(),
            }
            offset += size + 2;
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.fd.sync_all()?;
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        todo!()
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let entry = LogEntry(self.total_log, LogOperation::Set(key.clone(), val));

        self.total_log += 1;
        let entry_bytes = serde_cbor::to_vec(&entry)?;
        // use 2 bytes to store the entry size using u16
        // if entry size is bigger than 65535, then we will panic
        assert!(
            entry_bytes.len() < 65535,
            "do not support entry size bigger than 64Kb"
        );

        let mut buf = vec![0; 0];
        buf.write_u16::<LittleEndian>(entry_bytes.len() as u16)?;
        self.fd.write(&buf)?;
        let written_bytes = self.fd.write(&entry_bytes)? + 2;
        let log_ptr = LogPointer(self.total_bytes, written_bytes);
        self.total_bytes += written_bytes;
        self.index.insert(key, log_ptr);
        self.save_metadata()?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }
}
