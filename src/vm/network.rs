use super::vminfo::*;

pub fn set_vmnetwork(seeds: i32, mac: &str) -> VMnetowrk {
    let (remainder, quotient) = calculate_mod_and_divide(seeds);

    // register_network(seeds, remainder, quotient);    //System level registration

    let netowrk = VMnetowrk::new(
        format!("172.16.{}.{}", quotient, remainder),
        format!("net{}", seeds),
        mac.to_string(),
        format!("tap{}", seeds),
    );

    // println!("VM network: {:#?}", netowrk);
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

fn derigister_network(seeds: i32) {
    //Just generated
    //this need the proper access with sudo, I think it's better to grant the ip command previleges
    // Generate the proper network configuration
    let tap_dev = format!("tap{}", seeds);

    // Shell commands
    let commands = [format!("sudo ip link del {}", tap_dev)];

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

fn register_network(seeds: i32, remainder: i32, quotient: i32) {
    //this need the proper access with sudo, I think it's better to grant the ip command previleges
    // Generate the proper network configuration
    let tap_dev = format!("tap{}", seeds);
    let tap_ip = format!("172.16.{}.{}", quotient, remainder);
    let mask_short = String::from("/24");

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
