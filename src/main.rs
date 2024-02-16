mod api;
mod fuse;
mod vm;
mod security;

fn main() {
    println!("Hello, world!");
    println!("{}", vm::vminfo::generate_uuid());

    async_main();   // for the tokio::main

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}

#[tokio::main]
async fn async_main() {
    if let Err(err) = api::firecrackerapi::set_boot_source().await {
        eprintln!("Error: {}", err);
    }

    // send_request("Paused", "").await?; // Send PATCH request to pause VM
    // send_request("Resumed", "").await?; // Send PATCH request to resume VM
    // send_request("", "Full").await?; // Send PUT request for full snapshot
    // send_request("", "Diff").await?; // Send PUT request for diff snapshot

}
