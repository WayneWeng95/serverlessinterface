enum vm_status {
    Initializaing,
    Running,
    Paused,
    Terminated,
}

struct vm_info {
    pid: u32,
    imageid: u32,
    network: String,
    status: vm_status,
    config: vm_firecracker_config,
}

struct vm_firecracker_config {
    kernel_image_path: String,
    boot_args: String,
    rootfs_path: String,
    vcpu_count: u32,
    mem_size_mib: u32,
    socket_path: String,
}

struct vm_state_resource {
    cpu: u32,
    memory: u32,
    storage: u32,
}

use std::collections::HashMap;

fn save_into_hashmap() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
}

use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    let uuid = uuid::Uuid::new_v4();
    // println!("{}", uuid);
    uuid
}
