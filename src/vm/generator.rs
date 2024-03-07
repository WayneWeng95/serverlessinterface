use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    let uuid = uuid::Uuid::new_v4();
    // println!("{}", uuid);
    uuid
}

use rand::{distributions::Standard, Rng};

pub fn generate_random_mac() -> String {
    let mut rng = rand::thread_rng();
    let mac_bytes: [u8; 6] = rng.sample(Standard);
    let mac_address = format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]
    );
    mac_address
}