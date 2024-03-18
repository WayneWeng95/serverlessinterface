use chacha20poly1305::consts::True;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::vm::vminfo::{IpLibrary, VmInfo, VmSetUp};
use crate::vm::{network, vmconfig};
use std::process::Command;

// let body = format!(r#"{{                 //The format version of the call body
//     "kernel_image_path": "{}",
//     "boot_args": "{}"
// }}"#, kernel_image_path, boot_args);

pub async fn initialize_vm(vmsetup: &VmSetUp, uid: i32) -> io::Result<()> {
    // let vminfo = VmInfo::new(vmsetup.uuid, image, network, status, config)
    // let microseconds = 100;
    // let duration = Duration::from_micros(microseconds);
    // tokio::time::sleep(duration).await;
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
            match set_rootfs(
                &vmsetup.socket_path,
                &vmsetup.rootfs_path,
                vmsetup.is_read_only,
            )
            .await
            {
                Ok(_) => {
                    println!("Rootfs set successfully");
                    // let vmnetwork = network::set_vmnetwork(uid, &vmsetup.mac_address);
                    // match set_network(
                    //     &vmsetup.socket_path,
                    //     &vmnetwork.iface_id,
                    //     &vmnetwork.guest_mac,
                    //     &vmnetwork.host_dev_name,
                    // )
                    // .await
                    // {
                    //     Ok(_) => {
                    //         println!("Network set successfully");
                    //     }
                    //     Err(e) => eprintln!("Error setting network: {}", e),
                    // }
                }
                Err(e) => eprintln!("Error setting rootfs: {}", e),
            }
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
    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?; //This one appears with similiar problem?
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
    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    Ok(())
}

async fn set_network(
    socket_path: &str,
    iface_id: &str,
    guest_mac: &str,
    host_dev_name: &str,
) -> io::Result<()> {
    let body = format!(
        r#"{{
        "iface_id": "{}",
        "guest_mac": "{}",
        "host_dev_name": "{}"
    }}"#,
        iface_id, guest_mac, host_dev_name
    );

    let mut stream = UnixStream::connect(socket_path).await?;

    let request = format!(
        "PUT /network-interfaces/{} HTTP/1.1\r\n\
                                Host: localhost\r\n\
                                Content-Type: application/json\r\n\
                                Content-Length: {}\r\n\
                                \r\n\
                                {}",
        iface_id,
        body.len(),
        body
    );

    stream.write_all(request.as_bytes()).await?;
    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    Ok(())
}

async fn set_machine_config(socket_path: &str) -> io::Result<()> {
    // let body = format!();

    let mut stream = UnixStream::connect(socket_path).await?;

    // let request = format!();

    // stream.write_all(request.as_bytes()).await?;
    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    Ok(())
} //the machine configuration

async fn set_log_file(socket_path: &str) -> io::Result<()> {
    // let body = format!();

    let mut stream = UnixStream::connect(socket_path).await?;

    // let request = format!();

    // stream.write_all(request.as_bytes()).await?;
    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    Ok(())
} //the machine configuration

use crate::vm::vminfo::VmStatus;
pub async fn instance_control(socket_path: &str, state: VmStatus) -> io::Result<()> {
    // Define the Unix socket path

    // Define the request body
    let (body, len) = match state {
        VmStatus::Running => {
            let body = r#"{"action_type": "InstanceStart"}"#;
            (body, body.len())
        }
        VmStatus::Paused => {
            let body = r#"{"state": "Paused"}"#;
            (body, body.len())
        }
        VmStatus::Resume => {
            let body = r#"{"state": "Resumed"}"#;
            (body, body.len())
        }
        VmStatus::Terminated => {
            let body = r#"{"action_type": "SendCtrlAltDel"}"#;
            (body, body.len())
        }
        _ => {
            println!("Invalid state");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid state"));
        }
    };

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
        len, body
    );

    // Send the request
    stream.write_all(request.as_bytes()).await?;

    sleep(Duration::from_micros(50)).await; //Add a delay to avoid all the request sent at the same time

    // Read the response
    // let mut response = String::new();
    // stream.read_to_string(&mut response).await?;        //This one appears with similiar problem?
    // println!("{}", response);

    Ok(())
}

pub async fn set_log() {}
