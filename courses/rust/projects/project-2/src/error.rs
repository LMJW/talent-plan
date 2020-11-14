use std::convert::From;

#[derive(Debug)]
pub enum KvErr {
    IOError(std::io::Error),
    ParseError(serde_cbor::error::Error),
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
