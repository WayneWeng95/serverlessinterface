#[derive(Debug)] // for debug
pub enum VmStatus {
    Initializaing,
    Ready,
    Running,
    Paused,
    Terminated,
}

#[derive(Debug)] // for debug
pub struct VmInfo {
    pub uuid: Uuid,
    image: String,
    network: String,
    status: VmStatus,
    config: VmFirecrackerConfig,
}

impl VmInfo {
    pub fn new(
        uuid: Uuid,
        image: String,
        network: String,
        status: VmStatus,
        config: VmFirecrackerConfig,
    ) -> Self {
        VmInfo {
            uuid,
            image,
            network,
            status,
            config,
        }
    }
}

#[derive(Debug)] // for debug
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

struct VmRuntime {
    cpu: u32,
    memory: u32,
    storage: u32,
    pid: u32,
    VM: VmInfo,
}

impl VmRuntime {
    pub fn new(cpu: u32, memory: u32, storage: u32, pid: u32, VM: VmInfo) -> Self {
        VmRuntime {
            cpu,
            memory,
            storage,
            pid,
            VM,
        }
    }

    pub fn update(&mut self, cpu: u32, memory: u32, storage: u32, pid: u32) {       //Check whether update the vm state or not
        self.cpu = cpu;
        self.memory = memory;
        self.storage = storage;
        self.pid = pid;
    }
}

pub fn net_work() -> String {
    let net = "eth0";
    net.to_string()
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
