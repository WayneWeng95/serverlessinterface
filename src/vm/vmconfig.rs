use super::vminfo::*;

fn set_up_vm() {
    let uuid = generate_uuid();

    let config = VmFirecrackerConfig::new(
        "/home/shared/images/vmlinux-5.10.198".to_string(),
        "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        "/home/shared/images/ubuntu-22.04.ext4".to_string(),
        1,
        1024,
        "/tmp/firecracker.socket".to_string(),
    );
}
