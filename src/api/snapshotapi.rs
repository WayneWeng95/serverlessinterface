//! This module contains functions for sending requests to Firecracker via Unix domain sockets.
//!
//! Functions:
//! - `send_request`: Sends a request to Firecracker using a Unix domain socket.
//! Four request types are supported:
//!    - `Paused`:  Pause the VM.
//!    - `Resumed`: Resume the VM.
//!    - `Full`: Create a full snapshot.
//!    - `Diff`: Create a diff snapshot.

use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::time::sleep;
use tokio::time::Duration;
use uuid::Uuid;

pub async fn snapshot_request(
    socket_path: &str,
    state: &str,
    uuid: Uuid,
    snapshot_type: &str,
    snapshot_path: &str,
) -> io::Result<()> {
    // Define the request body
    let body = match snapshot_type {
        "Full" => format!(
            r#"{{ "snapshot_type": "{}", "snapshot_path": "./{}/snapshot_{}", "mem_file_path": "./{}/mem_file_{}" }}"#,
            snapshot_type, snapshot_path, uuid, snapshot_path, uuid
        ),
        "Diff" => format!(
            r#"{{ "snapshot_type": "{}", "snapshot_path": "./{}/snapshot_{}", "mem_file_path": "./{}/mem_file_{}" }}"#,
            snapshot_type, snapshot_path, uuid, snapshot_path, uuid
        ),
        _ => format!(r#"{{ "state": "{}" }}"#, state),
    };

    // Establish a connection to the Unix domain socket
    let mut stream = UnixStream::connect(socket_path).await?;

    // Construct the HTTP request
    let request = match snapshot_type {
        "Full" | "Diff" => format!(
            "PUT /snapshot/create HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
            body.len(),
            body
        ),
        _ => format!(
            "PATCH /vm HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
            body.len(),
            body
        ),
    };

    // Send the request
    stream.write_all(request.as_bytes()).await?;
    sleep(Duration::from_micros(300)).await; //Add a delay to avoid all the request

    Ok(())
}
