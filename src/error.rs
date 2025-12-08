//! IPC client error types

#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error("Failed to connect to kodegend daemon: {0}")]
    ConnectionFailed(std::io::Error),

    #[error("Socket path not found")]
    SocketPathNotFound,

    #[error("Failed to serialize message: {0}")]
    SerializationFailed(serde_json::Error),

    #[error("Failed to deserialize message: {0}")]
    DeserializationFailed(serde_json::Error),

    #[error("Message too large: {0} bytes (max: 1MB)")]
    MessageTooLarge(usize),

    #[error("Failed to write to stream: {0}")]
    WriteFailed(std::io::Error),

    #[error("Failed to read from stream: {0}")]
    ReadFailed(std::io::Error),

    #[error("Failed to flush stream: {0}")]
    FlushFailed(std::io::Error),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),
}
