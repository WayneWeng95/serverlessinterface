use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

// let body = format!(r#"{{                 //The format version of the call body
//     "kernel_image_path": "{}",
//     "boot_args": "{}"
// }}"#, kernel_image_path, boot_args);

pub async fn initialize_vm() -> io::Result<()> {
    set_boot_source().await?;
    set_rootfs().await?;
    start_instance().await?;
    Ok(())
}

pub async fn set_boot_source() -> io::Result<()> {
    // Define the Unix socket path
    let socket_path = "/tmp/firecracker.socket";

    // Define the request body
    let body = r#"{
        "kernel_image_path": "/home/shared/images/vmlinux-5.10.198",
        "boot_args": "console=ttyS0 reboot=k panic=1 pci=off"
    }"#;

    // let body = format!(r#"{{                 //The format version of the call body
    //     "kernel_image_path": "{}",
    //     "boot_args": "{}"
    // }}"#, kernel_image_path, boot_args);

    // Establish a connection to the Unix domain socket
    let mut stream = UnixStream::connect(socket_path).await?;

    // Construct the HTTP request
    let request = format!(
        "PUT /boot-source HTTP/1.1\r\n\
                            Host: localhost\r\n\
                            Accept: application/json\r\n\
                            Content-Type: application/json\r\n\
                            Content-Length: {}\r\n\
                            \r\n\
                            {}",
        body.len(),
        body
    );

    // Send the request
    stream.write_all(request.as_bytes()).await?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("{}", response);

    Ok(())
}

pub async fn set_rootfs() -> io::Result<()> {
    // Define the Unix socket path
    let socket_path = "/tmp/firecracker.socket";

    // Define the request body
    let body = r#"{
        "drive_id": "rootfs",
        "path_on_host": "/home/shared/images/ubuntu-22.04.ext4",
        "is_root_device": true,
        "is_read_only": false
    }"#;

    // Establish a connection to the Unix domain socket
    let mut stream = UnixStream::connect(socket_path).await?;

    // Construct the HTTP request
    let request = format!(
        "PUT /drives/rootfs HTTP/1.1\r\n\
                            Host: localhost\r\n\
                            Accept: application/json\r\n\
                            Content-Type: application/json\r\n\
                            Content-Length: {}\r\n\
                            \r\n\
                            {}",
        body.len(),
        body
    );

    // Send the request
    stream.write_all(request.as_bytes()).await?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("{}", response);

    Ok(())
}

async fn start_instance() -> io::Result<()> {
    // Define the Unix socket path
    let socket_path = "/tmp/firecracker.socket";

    // Define the request body
    let body = r#"{
        "action_type": "InstanceStart"
    }"#;

    // Establish a connection to the Unix domain socket
    let mut stream = UnixStream::connect(socket_path).await?;

    // Construct the HTTP request
    let request = format!(
        "PUT /actions HTTP/1.1\r\n\
                            Host: localhost\r\n\
                            Accept: application/json\r\n\
                            Content-Type: application/json\r\n\
                            Content-Length: {}\r\n\
                            \r\n\
                            {}",
        body.len(),
        body
    );

    // Send the request
    stream.write_all(request.as_bytes()).await?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("{}", response);

    Ok(())
}
