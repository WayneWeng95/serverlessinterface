mod api;
mod fuse;
mod security;
mod vm;

fn main() {
    println!("Hello, world!");
    println!("{}", vm::vminfo::generate_uuid());

    test_main();

    async_main(); // for the tokio::main

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}

fn test_main() {
    //functional tests
    let vmsetup = vm::vminfo::VmSetUp::default_test();

    println!("VM setup: {:#?}", vmsetup);
}

#[tokio::main]
async fn async_main() {
    if let Err(err) = vm::vmconfig::set_up_vm().await {
        eprintln!("Error: {}", err);
    }

    // send_request("Paused", "").await?; // Send PATCH request to pause VM
    // send_request("Resumed", "").await?; // Send PATCH request to resume VM
    // send_request("", "Full").await?; // Send PUT request for full snapshot
    // send_request("", "Diff").await?; // Send PUT request for diff snapshot
}
