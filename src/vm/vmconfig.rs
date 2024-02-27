use super::vminfo::*;

pub fn set_up_vm() {
    let uuid = generate_uuid();

    let config = VmSetUp::new(
        "/home/shared/images/vmlinux-5.10.198".to_string(),
        "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        "/home/shared/images/ubuntu-22.04.ext4".to_string(),
        false,
        1,
        128,
    );

    let vm = VmInfo::new(
        uuid,
        "/home/shared/images/ubuntu-22.04.ext4".to_string(),
        vm_network(),
        VmStatus::Initializaing,
        config,
    );

    println!("VM {:#?}", vm);
}

pub fn vm_network() -> String {
    let net = "eth0";
    net.to_string()
}

pub fn vm_runtime() {}
