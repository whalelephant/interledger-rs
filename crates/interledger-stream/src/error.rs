/// Stream Errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error connecting: {0}")]
    ConnectionError(String),
    #[error("Error polling: {0}")]
    PollError(String),
    #[error("Error polling: {0}")]
    SendMoneyError(String),
    #[error("Error maximum time exceeded: {0}")]
    TimeoutError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidPacket {
    #[error("Error OER: {0}")]
    OerErr(#[from] std::io::Error),
    #[error("Decryption Error")]
    FailedToDecrypt,
    #[error("Error STREAM Version: {0}")]
    InvalidStreamVersion(u8),
    #[error("Unknown or Unexpected Packet Type")]
    WrongPacketType,
    #[error("Incorrect number of parsed frames")]
    IncorrectFrameCount,
}
