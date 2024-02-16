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

async fn snapshot_request(state: &str, snapshot_type: &str) -> io::Result<()> {
    // Define the Unix socket path
    let socket_path = "/tmp/firecracker.socket";

    // Define the request body
    let body = match snapshot_type {
        "Full" => format!(
            r#"{{ "snapshot_type": "{}", "snapshot_path": "./snapshot_file", "mem_file_path": "./mem_file" }}"#,
            snapshot_type
        ),
        "Diff" => format!(
            r#"{{ "snapshot_type": "{}", "snapshot_path": "./snapshot_file", "mem_file_path": "./mem_file" }}"#,
            snapshot_type
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

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("{}", response);

    Ok(())
}
