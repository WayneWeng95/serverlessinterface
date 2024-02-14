mod chunks;
mod encrypt;
mod fuse;
mod vminfo;

fn main() {
    println!("Hello, world!");
    println!("{}", generate_uuid());

    // chunks::chunks_cutting().unwrap();

    // blocks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}
