use super::network;
use super::vminfo::*;
use crate::api;
use crate::vm;
use std::io;

use std::os::linux::net;
use std::time::Duration;
use tokio::time::sleep;

pub async fn set_up_vm(vm: vm::vminfo::VmSetUp, uid: i32) -> io::Result<()> {

    match api::firecrackerapi::initialize_vm(&vm, uid).await {
        Ok(_) => {
            println!("VM configured successfully");
            api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Running).await?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    sleep(Duration::from_secs(300)).await; // 5 minute before self destruction
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Terminated).await?;

    Ok(())
}

pub async fn end_vm(vm: vm::vminfo::VmSetUp, uid: i32) -> io::Result<()> {
    api::firecrackerapi::instance_control(&vm.socket_path, VmStatus::Terminated).await?;

    Ok(())
}
