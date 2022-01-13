use thiserror::Error;

#[derive(Error, Debug)]
// ErrorKind
pub enum Error {
    #[error("connection failure: {0}")]
    ConnectionFailure(String),

    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
