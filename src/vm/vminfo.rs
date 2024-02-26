enum VmStatus {
    Initializaing,
    Ready,
    Running,
    Paused,
    Terminated,
}

struct VmInfo {
    imageid: u32,
    network: String,
    status: VmStatus,
    config: VmFirecrackerConfig,
}

pub struct VmFirecrackerConfig {
    kernel_image_path: String,
    boot_args: String,
    rootfs_path: String,
    vcpu_count: u32,
    mem_size_mib: u32,
    socket_path: String,
}

impl VmFirecrackerConfig {
    pub fn new(
        kernel_image_path: String,
        boot_args: String,
        rootfs_path: String,
        vcpu_count: u32,
        mem_size_mib: u32,
        socket_path: String,
    ) -> Self {
        VmFirecrackerConfig {
            kernel_image_path,
            boot_args,
            rootfs_path,
            vcpu_count,
            mem_size_mib,
            socket_path,
        }
    }
}

struct VmStateResource {
    cpu: u32,
    memory: u32,
    storage: u32,
    pid: u32,
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
