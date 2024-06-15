use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PineError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Bincode error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Path conversion error")]
    PathConversion,
}
