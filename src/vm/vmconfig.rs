use super::network;
use super::vminfo::*;
use crate::api;
use crate::vm;
use std::io;

use std::os::linux::net;
use std::time::Duration;
use tokio::time::sleep;

pub async fn set_up_vm(uid: i32, vm: vm::vminfo::VmSetUp) -> io::Result<()> {
    match api::firecrackerapi::initialize_vm(&vm, uid).await {
        Ok(_) => {
            println!("VM configured successfully");
            start_vm(vm).await?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

pub async fn start_vm(vm: vm::vminfo::VmSetUp) -> io::Result<()> {
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Running).await?;
    sleep(Duration::from_secs(500)).await; // 5 minute before self destruction
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Terminated).await?;

    Ok(())
}

pub async fn end_vm(vm: vm::vminfo::VmSetUp) -> io::Result<()> {
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Terminated).await?;

    Ok(())
}

pub async fn snapshot_vm(vm: vm::vminfo::VmSetUp, snapshot_type: &str) -> io::Result<()> {
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Paused).await?;

    sleep(Duration::from_micros(100)).await; // 5 minute before self destruction

    api::snapshotapi::snapshot_request(&vm.socket_path, "Full", snapshot_type).await?;

    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Resume).await?;

    Ok(())
}
