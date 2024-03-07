#[derive(Debug)] // for debug
pub struct VmSetUp {
    pub uid: i32,
    pub uuid: Uuid,
    pub socket_path: String,
    pub mac_address: String,
    pub kernel_image_path: String,
    pub boot_args: String,
    pub rootfs_path: String,
    pub is_read_only: bool,
    pub vcpu_count: u32,
    pub mem_size_mib: u32,
}

impl VmSetUp {
    pub fn new(
        uid: i32,
        kernel_image_path: String,
        boot_args: String,
        rootfs_path: String,
        is_read_only: bool,
        vcpu_count: u32,
        mem_size_mib: u32,
    ) -> Self {
        let uuid = generate_uuid();
        // let socket_path = format!("/tmp/firecracker.socket");
        let socket_path = format!("/tmp/firecracker_{}.socket", uid);
        let mac_address = generate_random_mac();
        VmSetUp {
            uid,
            uuid,
            mac_address,
            socket_path,
            kernel_image_path,
            boot_args,
            rootfs_path,
            is_read_only,
            vcpu_count,
            mem_size_mib,
        }
    }

    pub fn default_test(uid: i32) -> Self {
        let uuid = generate_uuid();
        // let socket_path = format!("/tmp/firecracker.socket");
        let socket_path = format!("/tmp/firecracker_{}.socket", uid);
        let mac_address = generate_random_mac();
        Self {
            uid: uid,
            uuid: uuid,
            mac_address: mac_address,
            socket_path: socket_path,
            kernel_image_path: "/home/shared/images/vmlinux-5.10.198".to_string(),
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
    Resume,
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

#[derive(Debug)] // for debug
pub struct VMnetowrk {
    pub ip: String,            //
    pub iface_id: String,      //netx
    pub guest_mac: String,     //MAC
    pub host_dev_name: String, //tapx
}

impl VMnetowrk {
    pub fn new(ip: String, iface_id: String, guest_mac: String, host_dev_name: String) -> Self {
        VMnetowrk {
            ip,
            iface_id,
            guest_mac,
            host_dev_name,
        }
    }
}

use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};
#[derive(Debug)] // for debug
pub struct IpLibrary {
    pub seeds: i32,
    pub freelist: Arc<Mutex<LinkedList<i32>>>,
}

impl IpLibrary {
    pub fn new() -> Self {
        IpLibrary {
            seeds: 1, // Initialize seeds to 1
            freelist: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    pub fn pop_freelist_or_seeds(&mut self) -> i32 {
        // Lock the Mutex to access the freelist safely
        let mut freelist = self.freelist.lock().unwrap();

        // Check if the freelist is empty
        if let Some(first_item) = freelist.pop_front() {
            first_item
        } else {
            // If freelist is empty, use seeds and increment it
            let seeds = self.seeds;
            self.seeds += 1;
            seeds
        }
    }

    pub fn relase_ip(&mut self, ip: i32) {
        // Lock the Mutex to access the freelist safely
        let mut freelist = self.freelist.lock().unwrap();

        // Add the IP to the freelist
        freelist.push_back(ip);
    }
}

struct VmRuntime {
    cpu: u32,
    memory: u32,
    storage: u32,
    pid: u32,
    vm: VmInfo,
}

impl VmRuntime {
    pub fn new(cpu: u32, memory: u32, storage: u32, pid: u32, vm: VmInfo) -> Self {
        VmRuntime {
            cpu,
            memory,
            storage,
            pid,
            vm,
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

use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    let uuid = uuid::Uuid::new_v4();
    // println!("{}", uuid);
    uuid
}

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
