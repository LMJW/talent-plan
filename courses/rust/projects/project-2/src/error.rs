use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum KvErr {
    IOError(std::io::Error),
    ParseError(serde_cbor::error::Error),
    KeyNotExistError,
}

impl From<std::io::Error> for KvErr {
    fn from(err: std::io::Error) -> Self {
        KvErr::IOError(err)
    }
}

impl From<serde_cbor::error::Error> for KvErr {
    fn from(err: serde_cbor::error::Error) -> Self {
        KvErr::ParseError(err)
    }
}

impl fmt::Display for KvErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvErr::IOError(e) => write!(f, "{}", e),
            KvErr::ParseError(e) => write!(f, "{}", e),
            KvErr::KeyNotExistError => write!(f, "Key not found"),
        }
    }
}
