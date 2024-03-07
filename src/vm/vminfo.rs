use uuid::Uuid;

#[derive(Debug, Clone)] // for debug
pub struct VmSetUp {
    pub uid: i32,
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
        uid: i32,
        uuid: Uuid,
        kernel_image_path: String,
        boot_args: String,
        rootfs_path: String,
        is_read_only: bool,
        vcpu_count: u32,
        mem_size_mib: u32,
    ) -> Self {
        // let socket_path = format!("/tmp/firecracker.socket");
        let socket_path = format!("/tmp/firecracker_{}.socket", uid);
        VmSetUp {
            uid,
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

    pub fn default_test(uid: i32, uuid: Uuid) -> Self {
        // let socket_path = format!("/tmp/firecracker.socket");
        let socket_path = format!("/tmp/firecracker_{}.socket", uid);
        Self {
            uid: uid,
            uuid: uuid,
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

#[derive(Debug, Clone)] // for debug
pub enum VmStatus {
    Initializaing,
    Ready,
    Running,
    Paused,
    Resume,
    Terminated,
}

#[derive(Debug, Clone)] // for debug
pub struct VmInfo {
    uid: i32,
    uuid: Uuid,
    image: String, //This is not in use now
    network: VMnetowrk,
    status: VmStatus,
    config: VmSetUp,
}

impl VmInfo {
    pub fn new(
        uid: i32,
        uuid: Uuid,
        image: String,
        network: VMnetowrk,
        status: VmStatus,
        config: VmSetUp,
    ) -> Self {
        VmInfo {
            uid,
            uuid,
            image,
            network,
            status,
            config,
        }
    }
}

#[derive(Debug, Clone)] // for debug
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
