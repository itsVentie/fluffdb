use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Data corruption detected (CRC mismatch)")]
    CorruptedData,
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
