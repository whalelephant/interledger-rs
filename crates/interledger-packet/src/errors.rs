use std::str::Utf8Error;
use std::string::FromUtf8Error;

use super::AddressError;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("I/O Error: {0}")]
    IoErr(#[from] std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8Err(#[from] Utf8Error),
    #[error("UTF-8 Conversion Error: {0}")]
    FromUtf8Err(#[from] FromUtf8Error),
    #[error("Chrono Error: {0}")]
    ChronoErr(#[from] chrono::ParseError),
    #[error("Packet Type Error: {reason:?}, {found:?} was found")]
    PacketTypeErr {
        reason: PacketTypeErrors,
        found: Option<u8>,
    },
    #[error("Invalid Address: {0}")]
    InvalidAddress(#[from] AddressError),
    // TODO: remove String
    #[error("Invalid Data Format for {target}: should be {expected}")]
    InvalidDataFormat { target: String, expected: String },
    #[error("Invalid Packet: {0}")]
    InvalidPacket(String),
    #[error("Packet Length Err: trailing bytes in {0:?}")]
    PacketLengthErr(Field),
    #[error("FuzzErr: {0}")]
    FuzzErr(String),
}

#[derive(Debug)]
pub enum Field {
    Content,
    Frame,
}

#[derive(Debug)]
pub enum PacketTypeErrors {
    MismatchExpected(u8),
    Unknown,
}

// TODO: doc
impl PacketTypeErrors {
    pub fn with_found(self, found: u8) -> ParseError {
        ParseError::PacketTypeErr {
            reason: self,
            found: Some(found),
        }
    }
    pub fn with_eof(self) -> ParseError {
        ParseError::PacketTypeErr {
            reason: self,
            found: None,
        }
    }
}
