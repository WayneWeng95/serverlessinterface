mod api;
mod fuse;
mod security;
mod vm;

use crate::fuse::fuse::fuse_main;
use crate::vm::network;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::task;
use vm::vminfo::{self, IpLibrary};

fn metadata(
    iplibrary: Arc<Mutex<IpLibrary>>,
    vm_map: Arc<Mutex<HashMap<i32, vm::vminfo::VmInfo>>>,
) -> (i32, vm::vminfo::VmSetUp) {
    let uid = iplibrary.lock().unwrap().pop_freelist_or_seeds();
    let mut map = vm_map.lock().unwrap();
    let mac = vm::generator::generate_random_mac();
    let uuid = vm::generator::generate_uuid();
    let network = network::set_vmnetwork(uid, &mac);
    let vm = vm::vminfo::VmSetUp::default_test(uid, uuid);
    let vminfo = vminfo::VmInfo::new(
        uid,
        uuid,
        "image".to_string(),
        network,
        vm::vminfo::VmStatus::Initializaing,
        vm.clone(),
    );

    map.insert(uid, vminfo);
    (uid, vm)
}

fn main() {
    println!("Hello, world!");
    // api::systemapi::remove_socket_files();

    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     println!("Usage: cargo run -- <input_variable>");
    //     return;
    // }

    // let input_variable: i32 = match args[1].parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Input variable must be an integer");
    //         return;
    //     }
    // };

    // let num_threads = input_variable; // Number of threads to spawn

    // let vm_map: Arc<Mutex<HashMap<i32, vm::vminfo::VmInfo>>> = Arc::new(Mutex::new(HashMap::new()));

    // let iplibrary = Arc::new(Mutex::new(vm::vminfo::IpLibrary::new()));

    // // let num_threads = 5; // Number of threads to spawn

    // let handles: Vec<_> = (0..num_threads)
    //     .map(|_| {
    //         let iplibrary_clone = Arc::clone(&iplibrary);
    //         let vm_map_clone = Arc::clone(&vm_map);

    //         thread::spawn(move || {
    //             let (uid, vm) = metadata(Arc::clone(&iplibrary_clone), Arc::clone(&vm_map_clone));
    //             api::systemapi::start_firecracker(vm.socket_path.clone());
    //             async_main(uid, vm);
    //         })
    //     })
    //     .collect();

    // // Wait for all threads to finish
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // let mut iplibrary = vm::vminfo::IpLibrary::new();

    // test_main(1); //testing with 1

    // // network::network_generate(iplibrary);        //done testing

    // let uid = iplibrary.pop_freelist_or_seeds();

    // let vm = vm::vminfo::VmSetUp::default_test(uid);

    // async_main(vm, uid); // for the tokio::main

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    fuse_main();

    // encrypt::crypto_demo().unwrap();
}

#[tokio::main]
async fn async_main(uid: i32, vm: vm::vminfo::VmSetUp) {
    // let socket = vm.socket_path.clone();
    // let handle = task::spawn_blocking(|| {
    //     start_firecracker(socket);
    // });
    // let mut iplibrary = vm::vminfo::IpLibrary::new();

    // let uid1 = iplibrary.pop_freelist_or_seeds();

    // let vm1 = vm::vminfo::VmSetUp::default_test(uid1);

    // let socket = vm1.socket_path.clone();

    // start_firecracker(socket);

    // sleep(std::time::Duration::from_secs(1));

    let handle1 = tokio::spawn(async move {
        if let Err(err) = vm::vmconfig::set_up_vm(uid, vm).await {
            eprintln!("Error: {}", err);
        }
    });

    handle1.await.expect("Failed to wait for task 1");

    // if let Err(err) = vm::vmconfig::set_up_vm(iplibrary).await {
    //     eprintln!("Error: {}", err);
    // }

    // send_request("Paused", "").await?; // Send PATCH request to pause VM
    // send_request("Resumed", "").await?; // Send PATCH request to resume VM
    // send_request("", "Full").await?; // Send PUT request for full snapshot
    // send_request("", "Diff").await?; // Send PUT request for diff snapshot
}
