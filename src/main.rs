mod chunks;
mod encrypt;
mod fuse;
mod vminfo;
mod firecrackerapi;

fn main() {
    println!("Hello, world!");
    println!("{}", vminfo::generate_uuid());

    firecrackerapi::set_boot_source().unwrap();

    // chunks::chunks_cutting().unwrap();

    // chunks::chunks_restoring().unwrap();

    // fuse::fuse_main();

    // encrypt::crypto_demo().unwrap();
}
