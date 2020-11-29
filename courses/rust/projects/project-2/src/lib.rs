use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use serde_cbor;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;

pub mod error;
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
type LogSize = usize;

struct Log(LogSize, LogEntry);

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let pb = path.into();
        let p = if pb.is_dir() {
            let mut tmp = PathBuf::from(&pb);
            tmp.push("test.db");
            tmp
        } else {
            pb
        };

        let fd = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(p)?;

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

    fn save_metadata(&mut self) -> Result<()> {
        let metadata = Metadata {
            total_log: self.total_log,
            total_bytes: self.total_bytes,
        };
        let md = serde_cbor::to_vec(&metadata)?;
        let mut buf = vec![0u8; 1024];
        let mut head = &mut buf[0..4];
        head.write_u32::<LittleEndian>(md.len() as u32)?;

        buf[4..md.len() + 4].clone_from_slice(&md);

        self.fd.write_all_at(&buf, 0)?;
        Ok(())
    }

    fn replay_log(&mut self) -> Result<()> {
        let mut offset: usize = 1024;

        for _ in 0..self.total_log {
            let Log(size, lp) = self.read_log(offset)?;
            match lp.1 {
                LogOperation::Set(key, _) => {
                    self.index.insert(key, LogPointer(offset - 1024, size + 2));
                }
                LogOperation::Rm(key) => {
                    self.index.remove(&key);
                }
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
        if let Some(val) = self.index.get(&key) {
            let entry = self.read_log_entry(val.0 + 1024)?;
            match entry.1 {
                LogOperation::Set(_, val) => Ok(Some(val)),
                LogOperation::Rm(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let entry = LogEntry(self.total_log, LogOperation::Set(key.clone(), val));
        let entry_bytes = serde_cbor::to_vec(&entry)?;

        let log_ptr = self.insert_log(entry_bytes)?;
        self.index.insert(key, log_ptr);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let entry = LogEntry(self.total_log, LogOperation::Rm(key.clone()));
        let entry_bytes = serde_cbor::to_vec(&entry)?;

        self.insert_log(entry_bytes)?;
        match self.index.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KvErr::KeyNotExistError),
        }
    }

    pub fn compact(&mut self) -> Result<()> {
        let mut data = Vec::new();
        self.fd.read_to_end(&mut data)?;

        let mut new_index = HashMap::<String, LogPointer>::new();
        let mut new_total_log = 0usize;
        let mut new_total_byte = 0usize;
        let mut logs = Vec::<u8>::new();
        for (key, val) in self.index.iter() {
            let LogPointer(old_offset, old_size) = val;
            new_total_log += 1;
            let mut buf = vec![0u8; *old_size];

            self.fd.read_exact_at(&mut buf, *old_offset as u64 + 1024)?;
            new_index.insert(key.to_owned(), LogPointer(new_total_byte, *old_size));
            logs.append(&mut buf);
            new_total_byte += old_size;
        }
        self.index = new_index;
        self.total_log = new_total_log;
        self.total_bytes = new_total_byte;

        self.save_metadata()?;
        self.fd.write_at(&logs, 1024)?;
        self.fd.set_len(1024 + logs.len() as u64)?;
        Ok(())
    }

    fn insert_log(&mut self, log_entry: Vec<u8>) -> Result<LogPointer> {
        self.total_log += 1;
        // use 2 bytes to store the entry size using u16
        // if entry size is bigger than 65535, then we will panic
        assert!(
            log_entry.len() < 65535,
            "do not support entry size bigger than 64Kb"
        );
        let offset = self.total_bytes + 1024;
        let mut buf = vec![0; 0];
        buf.write_u16::<LittleEndian>(log_entry.len() as u16)?;
        self.fd.write_at(&buf, offset as u64)?;
        let written_bytes = self.fd.write_at(&log_entry, offset as u64 + 2)? + 2;
        self.total_bytes += written_bytes;
        self.save_metadata()?;
        Ok(LogPointer(offset - 1024, written_bytes))
    }

    fn read_log_pointer(&self, log_ptr: &LogPointer) -> Result<Log> {
        let LogPointer(offset, size) = log_ptr;
        let entry = self.read_log_entry_body(offset + 2, *size)?;
        Ok(Log(*size, entry))
    }

    fn read_log(&self, offset: usize) -> Result<Log> {
        let size = self.read_log_entry_head(offset)? as usize;
        let entry = self.read_log_entry_body(offset + 2, size)?;
        Ok(Log(size, entry))
    }

    fn read_log_entry(&self, offset: usize) -> Result<LogEntry> {
        let size = self.read_log_entry_head(offset)? as usize;
        Ok(self.read_log_entry_body(offset + 2, size)?)
    }

    fn read_log_entry_head(&self, offset: usize) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.fd.read_exact_at(&mut buf, offset as u64)?;
        let mut cursor = std::io::Cursor::new(&buf);
        Ok(cursor.read_u16::<LittleEndian>()?)
    }

    fn read_log_entry_body(&self, offset: usize, size: usize) -> Result<LogEntry> {
        let mut data = vec![0u8; size];
        self.fd.read_exact_at(&mut data, offset as u64)?;
        Ok(serde_cbor::from_reader(&data[..])?)
    }
}
