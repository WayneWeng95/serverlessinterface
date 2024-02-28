use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use crate::vm::vminfo::{VmInfo, VmSetUp};

// let body = format!(r#"{{                 //The format version of the call body
//     "kernel_image_path": "{}",
//     "boot_args": "{}"
// }}"#, kernel_image_path, boot_args);

pub async fn initialize_vm(vmsetup: VmSetUp) -> io::Result<()> {
    // let vminfo = VmInfo::new(vmsetup.uuid, image, network, status, config)
    match set_boot_source(
        &vmsetup.socket_path,
        &vmsetup.kernel_image_path,
        &vmsetup.boot_args,
    )
    .await
    {
        Ok(_) => {
            println!("Boot source set successfully");
            // vmsetup.vm_state = VmStatus::Initializaing;
            set_rootfs(
                &vmsetup.socket_path,
                &vmsetup.rootfs_path,
                vmsetup.is_read_only,
            )
            .await?;
            instance_control(VmStatus::Running).await?;
        }
        Err(e) => eprintln!("Error setting boot source: {}", e),
    }
    Ok(())
}

pub async fn set_boot_source(
    socket_path: &str,
    kernel_image_path: &str,
    boot_args: &str,
) -> io::Result<()> {
    // Define the Unix socket path

    // Define the request body
    let body = format!(
        r#"{{          
        "kernel_image_path": "{}",
        "boot_args": "{}"
    }}"#,
        kernel_image_path, boot_args
    );

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

pub async fn set_rootfs(
    socket_path: &str,
    rootfs_path: &str,
    is_read_only: bool,
) -> io::Result<()> {
    // Define the request body
    let body = format!(
        r#"{{          
        "drive_id": "rootfs",
        "path_on_host": "{}",
        "is_root_device": true,
        "is_read_only": {}
    }}"#,
        rootfs_path, is_read_only
    );

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

use crate::vm::vminfo::VmStatus;
async fn instance_control(state: VmStatus) -> io::Result<()> {
    // Define the Unix socket path
    let socket_path = "/tmp/firecracker/firecracker.socket";

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
