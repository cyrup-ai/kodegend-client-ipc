//! Cross-platform IPC client for kodegend daemon
//!
//! This library provides a high-level API for querying the kodegend daemon
//! via IPC (Unix sockets on Unix platforms, Named Pipes on Windows).

mod error;
mod protocol;
mod connection;

#[cfg(windows)]
mod windows_pipe;

pub use error::IpcError;
pub use kodegend_protocol_ipc::*;

use connection::connect_to_daemon;
use protocol::{send_message, recv_message};

/// Query kodegend daemon with a status query
pub fn query_daemon(query: StatusQuery) -> Result<StatusResponse, IpcError> {
    let mut stream = connect_to_daemon()?;
    send_message(&mut stream, &query)?;
    recv_message(&mut stream)
}

/// Get all service statuses
pub fn get_all_services() -> Result<Vec<ServiceStatus>, IpcError> {
    let response = query_daemon(StatusQuery::All)?;
    Ok(response.services)
}

/// Get specific service status
pub fn get_service(name: &str) -> Result<ServiceStatus, IpcError> {
    let response = query_daemon(StatusQuery::Service(name.to_string()))?;
    response.services
        .into_iter()
        .find(|s| s.name == name)
        .ok_or_else(|| IpcError::ServiceNotFound(name.to_string()))
}

/// Get aggregated usage statistics from all backend servers for a specific connection
pub fn get_usage_stats(connection_id: &str) -> Result<AggregatedUsageStats, IpcError> {
    let mut stream = connect_to_daemon()?;
    send_message(&mut stream, &StatusQuery::UsageStats(connection_id.to_string()))?;
    recv_message(&mut stream)
}

/// Get aggregated tool history from all backend servers for a specific connection
pub fn get_tool_history(connection_id: &str) -> Result<AggregatedToolHistory, IpcError> {
    let mut stream = connect_to_daemon()?;
    send_message(&mut stream, &StatusQuery::ToolHistory(connection_id.to_string()))?;
    recv_message(&mut stream)
}

/// Check if daemon is running
pub fn is_daemon_running() -> bool {
    connect_to_daemon().is_ok()
}
