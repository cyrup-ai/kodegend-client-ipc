//! Cross-platform connection to kodegend daemon

use crate::error::IpcError;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

#[cfg(windows)]
use crate::windows_pipe::{NamedPipeStream, connect_named_pipe};

/// Connect to kodegend daemon using platform-appropriate IPC transport
#[cfg(unix)]
pub(crate) fn connect_to_daemon() -> Result<UnixStream, IpcError> {
    let socket_path = get_socket_path()?;
    UnixStream::connect(socket_path)
        .map_err(IpcError::ConnectionFailed)
}

/// Connect to kodegend daemon using Windows Named Pipes
#[cfg(windows)]
pub(crate) fn connect_to_daemon() -> Result<NamedPipeStream, IpcError> {
    let pipe_path = get_pipe_path();
    connect_named_pipe(&pipe_path)
        .map_err(IpcError::ConnectionFailed)
}

/// Get Unix socket path for kodegend status socket
#[cfg(unix)]
fn get_socket_path() -> Result<PathBuf, IpcError> {
    // 1. Check KODEGEND_STATUS_SOCKET environment variable
    if let Ok(path) = std::env::var("KODEGEND_STATUS_SOCKET") {
        return Ok(PathBuf::from(path));
    }

    // 2. Try user-level socket first
    let user_socket = dirs::runtime_dir()
        .or_else(dirs::data_local_dir)
        .map(|dir| dir.join("kodegend").join("status.sock"));

    if let Some(path) = &user_socket
        && path.exists() {
            return Ok(path.clone());
        }

    // 3. Try system-level socket (for system-wide daemon)
    let system_socket = PathBuf::from("/var/run/kodegend/status.sock");
    if system_socket.exists() {
        return Ok(system_socket);
    }

    // 4. Fallback to user-level path (even if doesn't exist - connection will fail gracefully)
    user_socket.ok_or(IpcError::SocketPathNotFound)
}

/// Get Windows Named Pipe path for kodegend status pipe
#[cfg(windows)]
fn get_pipe_path() -> String {
    // Check environment variable first
    if let Ok(path) = std::env::var("KODEGEND_STATUS_PIPE") {
        return path;
    }

    // Standard Windows Named Pipe path
    r"\\.\pipe\kodegend\status".to_string()
}
