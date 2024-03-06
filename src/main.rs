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

    let mut iplibrary = vm::vminfo::IpLibrary::new();

    test_main(1); //testing with 1

    // network::network_generate(iplibrary);        //done testing

    let uid = iplibrary.pop_freelist_or_seeds();

    let vm = vm::vminfo::VmSetUp::default_test(uid);

    start_firecracker(uid);

    async_main(vm, uid); // for the tokio::main

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

use std::process::Command;
use std::thread::sleep;
fn start_firecracker(uid: i32) {
    // let socket = format!("/tmp/firecracker/No{}.socket", uid);
    let socket = format!("/tmp/firecracker.socket"); //simple demo
    Command::new("firecracker")
        .arg("--api-sock")
        .arg(socket)
        .output()
        .expect("Failed to execute firecracker command");

    sleep(std::time::Duration::from_micros(100)); //Add a delay
}

#[tokio::main] //temporarily comment out for testing
async fn async_main(vm: vm::vminfo::VmSetUp, uid: i32) {
    if let Err(err) = vm::vmconfig::set_up_vm(vm, uid).await {
        eprintln!("Error: {}", err);
    }

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
