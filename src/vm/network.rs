use super::vminfo::*;
use rand::{distributions::Standard, Rng};

fn generate_random_mac() -> String {
    let mut rng = rand::thread_rng();
    let mac_bytes: [u8; 6] = rng.sample(Standard);
    let mac_address = format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]
    );
    mac_address
}

use std::collections::HashMap;
pub fn network_generate(mut iplibrary: IpLibrary) -> VMnetowrk {
    let seeds = iplibrary.pop_freelist_or_seeds();
    let mac = generate_random_mac();
    let network = set_vmnetwork(seeds, &mac);
    iplibrary.insert_used(seeds, mac);
    println!("IP Library: {:#?}", iplibrary);
    network
}

fn set_vmnetwork(seeds: i32, mac: &str) -> VMnetowrk {
    let (remainder, quotient) = calculate_mod_and_divide(seeds);

    let netowrk = VMnetowrk::new(
        format!("172.16.{}.{}", quotient, remainder),
        format!("net{}", seeds),
        mac.to_string(),
        format!("tap{}", seeds),
    );
    println!("VM network: {:#?}", netowrk);
    netowrk
}

fn calculate_mod_and_divide(number: i32) -> (i32, i32) {
    // Calculate the remainder (modulus) when dividing by 255
    let remainder = number % 255;

    // Calculate the quotient when dividing by 255
    let quotient = number / 255;

    // Return the remainder and quotient as a tuple
    (remainder, quotient)
}

use std::process::Command;
fn register_network() {
    //this need the proper access with sudo, I think it's better to grant the ip command previleges
    // Generate the proper network configuration
    let tap_dev = format!("tap{}", 0);
    let tap_ip = std::env::var("TAP_IP").unwrap_or_else(|_| String::from("172.16.0.1"));
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
