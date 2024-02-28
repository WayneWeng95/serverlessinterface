use super::vminfo::*;
use crate::api;
use crate::vm;
use std::io;

pub async fn set_up_vm() -> io::Result<()> {
    let vmsetup = vm::vminfo::VmSetUp::default_test();

    match api::firecrackerapi::initialize_vm(&vmsetup).await {
        Ok(_) => {
            println!("VM configured successfully");
            api::firecrackerapi::instance_control(&vmsetup.socket_path, VmStatus::Running).await?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

pub fn vm_network() -> VMnetowrk {
    let netowrk = VMnetowrk::new(
        "net1".to_string(),
        "00:1a:4a:16:01:01".to_string(),
        "eth0".to_string(),
    );

    register_network();

    netowrk
}

use std::process::Command;
fn register_network() {
    //this need the proper access with sudo
    let tap_dev = std::env::var("TAP_DEV").unwrap_or_else(|_| String::from("tap0"));
    let tap_ip = std::env::var("TAP_IP").unwrap_or_else(|_| String::from("192.168.0.1"));
    let mask_short = std::env::var("MASK_SHORT").unwrap_or_else(|_| String::from("/24"));

    // Shell commands
    let commands = [
        format!("sudo ip link del {} 2> /dev/null || true", tap_dev),
        format!("sudo ip tuntap add dev {} mode tap", tap_dev),
        format!("sudo ip addr add {}{} dev {}", tap_ip, mask_short, tap_dev),
        format!("sudo ip link set dev {} up", tap_dev),
    ];

    // Execute each command
    for cmd in &commands {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn vm_runtime() {}
