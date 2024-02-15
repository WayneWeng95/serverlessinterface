mod chunks;
mod encrypt;
mod fuse;
mod vminfo;

fn main() {
    println!("Hello, world!");
    println!("{}", vminfo::generate_uuid());

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    encrypt::crypto_demo().unwrap();
}
