#[derive(Debug)] // for debug
pub struct VmSetUp {
    pub uuid: Uuid,
    pub socket_path: String,
    pub kernel_image_path: String,
    pub boot_args: String,
    pub rootfs_path: String,
    pub is_read_only: bool,
    pub vcpu_count: u32,
    pub mem_size_mib: u32,
}

impl VmSetUp {
    pub fn new(
        kernel_image_path: String,
        boot_args: String,
        rootfs_path: String,
        is_read_only: bool,
        vcpu_count: u32,
        mem_size_mib: u32,
    ) -> Self {
        let uuid = generate_uuid();
        let socket_path = format!("/tmp/firecracker_{}.socket", uuid.to_string());
        VmSetUp {
            uuid,
            socket_path,
            kernel_image_path,
            boot_args,
            rootfs_path,
            is_read_only,
            vcpu_count,
            mem_size_mib,
        }
    }

    pub fn default_test() -> Self {
        let uuid = generate_uuid();
        let socket_path = format!("/tmp/firecracker_{}.socket", uuid.to_string());
        Self {
            uuid: uuid,
            socket_path: socket_path,
            kernel_image_path: "/home/shared/images/kernel_image".to_string(),
            boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
            rootfs_path: "/home/shared/images/ubuntu-22.04.ext4".to_string(),
            is_read_only: false,
            vcpu_count: 1,
            mem_size_mib: 128,
        }
    }
}

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
    config: VmSetUp,
}

impl VmInfo {
    pub fn new(
        uuid: Uuid,
        image: String,
        network: String,
        status: VmStatus,
        config: VmSetUp,
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

    pub fn update(&mut self, cpu: u32, memory: u32, storage: u32, pid: u32) {
        //Check whether update the vm state or not
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
