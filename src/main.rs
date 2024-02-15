mod chunks;
mod encrypt;
mod firecrackerapi;
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
}
