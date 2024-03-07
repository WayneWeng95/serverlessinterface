use std::process::{Command, Stdio,ExitStatus};
use std::thread::sleep;
pub fn start_firecracker(socket: String) -> u32 {
    let child = Command::new("firecracker")
        .arg("--api-sock")
        .arg(socket)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute firecracker command");

    sleep(std::time::Duration::from_secs(1)); //Add a delay
                                              // Get the PID of the spawned process
    let pid = child.id();

    println!("PID of the spawned process: {}", pid);
    pid
}

pub fn remove_socket_files() -> ExitStatus {
    // Run the command
    Command::new("rm")
        .arg("/tmp/firecracker_*")
        .status()
        .expect("failed to execute process")
}

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
