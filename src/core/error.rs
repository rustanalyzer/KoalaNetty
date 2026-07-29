//! Error types for the KoalaNetty client

use thiserror::Error;

/// Main error type for the KoalaNetty client
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Server error: {type:?} - {message}")]
    Server {
        type_: Option<String>,
        message: String,
    },

    #[error("Invalid packet: {0}")]
    InvalidPacket(String),

    #[error("Not connected")]
    NotConnected,

    #[error("Channel send error: {0}")]
    ChannelSend(String),
}

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::InvalidPacket(err.to_string())
    }
}
