use crate::vm::network;
use tokio::task;
mod api;
mod fuse;
mod security;
mod vm;

use std::collections::HashMap;

fn save_into_hashmap() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
}

fn main() {
    println!("Hello, world!");
    println!("{}", vm::vminfo::generate_uuid());

    let mut map: HashMap<i32, vm::vminfo::VmSetUp> = std::collections::HashMap::new();

    // let mut iplibrary = vm::vminfo::IpLibrary::new();

    // test_main(1); //testing with 1

    // // network::network_generate(iplibrary);        //done testing

    // let uid = iplibrary.pop_freelist_or_seeds();

    // let vm = vm::vminfo::VmSetUp::default_test(uid);

    // async_main(vm, uid); // for the tokio::main

    async_main();

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}

fn test_main(uid: i32) {
    //functional tests
    let vmsetup = vm::vminfo::VmSetUp::default_test(uid);

    println!("VM setup: {:#?}", vmsetup);
}

use std::process::{Command, Stdio};
use std::thread::sleep;
fn start_firecracker(socket: String) {
    let child = Command::new("firecracker")
        .arg("--api-sock")
        .arg(socket)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute firecracker command");

    sleep(std::time::Duration::from_secs(1)); //Add a delay
                                              // Get the PID of the spawned process
    let pid = child.id();

    println!("PID of the spawned process: {}", pid);
}

#[tokio::main] //temporarily comment out for testing
async fn async_main() {
    // let socket = vm.socket_path.clone();
    // let handle = task::spawn_blocking(|| {
    //     start_firecracker(socket);
    // });
    let mut iplibrary = vm::vminfo::IpLibrary::new();

    let uid1 = iplibrary.pop_freelist_or_seeds();

    let vm1 = vm::vminfo::VmSetUp::default_test(uid1);

    let socket = vm1.socket_path.clone();

    start_firecracker(socket);

    sleep(std::time::Duration::from_secs(1));

    let handle1 = tokio::spawn(async move {
        if let Err(err) = vm::vmconfig::set_up_vm(vm1, uid1).await {
            eprintln!("Error: {}", err);
        }
    });

    // let uid2 = iplibrary.pop_freelist_or_seeds();

    // let vm2 = vm::vminfo::VmSetUp::default_test(uid2);

    // let handle2 = tokio::spawn(async move {
    //     if let Err(err) = vm::vmconfig::set_up_vm(vm2, uid2).await {
    //         eprintln!("Error: {}", err);
    //     }
    // });

    // let uid3 = iplibrary.pop_freelist_or_seeds();

    // let vm3 = vm::vminfo::VmSetUp::default_test(uid3);

    // let handle3 = tokio::spawn(async move {
    //     if let Err(err) = vm::vmconfig::set_up_vm(vm3, uid3).await {
    //         eprintln!("Error: {}", err);
    //     }
    // });

    handle1.await.expect("Failed to wait for task 1");
    // handle2.await.expect("Failed to wait for task 2");
    // handle3.await.expect("Failed to wait for task 3");

    // if let Err(err) = vm::vmconfig::set_up_vm(iplibrary).await {
    //     eprintln!("Error: {}", err);
    // }

    // send_request("Paused", "").await?; // Send PATCH request to pause VM
    // send_request("Resumed", "").await?; // Send PATCH request to resume VM
    // send_request("", "Full").await?; // Send PUT request for full snapshot
    // send_request("", "Diff").await?; // Send PUT request for diff snapshot
}

// async fn spawn_vm() {
//     if let Err(err) = vm::vmconfig::set_up_vm(iplibrary).await {     //think about how the network configuration works in this level
//         eprintln!("Error: {}", err);
//     }
// }
