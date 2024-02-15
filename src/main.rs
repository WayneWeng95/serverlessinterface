mod chunks;
mod encrypt;
mod firecrackerapi;
mod snapshotapi;
mod fuse;
mod vminfo;

fn main() {
    println!("Hello, world!");
    println!("{}", vminfo::generate_uuid());

    async_main();   // for the tokio::main

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}

#[tokio::main]
async fn async_main() {
    if let Err(err) = firecrackerapi::set_boot_source().await {
        eprintln!("Error: {}", err);
    }

    // send_request("Paused", "").await?; // Send PATCH request to pause VM
    // send_request("Resumed", "").await?; // Send PATCH request to resume VM
    // send_request("", "Full").await?; // Send PUT request for full snapshot
    // send_request("", "Diff").await?; // Send PUT request for diff snapshot

}
