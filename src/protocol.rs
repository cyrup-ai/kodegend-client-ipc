//! Wire protocol implementation (length-prefixed JSON)

use std::io::{Read, Write};
use serde::{Serialize, de::DeserializeOwned};
use crate::error::IpcError;

/// Maximum message size (1MB) to prevent DoS attacks
const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// Send a message over the stream (Unix socket or Windows Named Pipe)
pub(crate) fn send_message<T: Serialize, S: Write>(stream: &mut S, msg: &T) -> Result<(), IpcError> {
    let json = serde_json::to_vec(msg)
        .map_err(IpcError::SerializationFailed)?;

    if json.len() > MAX_MESSAGE_SIZE {
        return Err(IpcError::MessageTooLarge(json.len()));
    }

    let len = (json.len() as u32).to_le_bytes();
    stream.write_all(&len)
        .map_err(IpcError::WriteFailed)?;
    stream.write_all(&json)
        .map_err(IpcError::WriteFailed)?;
    stream.flush()
        .map_err(IpcError::FlushFailed)?;

    Ok(())
}

/// Receive a message from the stream
pub(crate) fn recv_message<T: DeserializeOwned, S: Read>(stream: &mut S) -> Result<T, IpcError> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)
        .map_err(IpcError::ReadFailed)?;

    let len = u32::from_le_bytes(len_bytes) as usize;

    if len > MAX_MESSAGE_SIZE {
        return Err(IpcError::MessageTooLarge(len));
    }

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)
        .map_err(IpcError::ReadFailed)?;

    serde_json::from_slice(&buf)
        .map_err(IpcError::DeserializationFailed)
}
