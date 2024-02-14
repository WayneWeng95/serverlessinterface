fn main() {
    println!("Hello, world!");
    println!("{}", generate_uuid());

    // snapshot_cutting2().unwrap();

    // blocks_restoring().unwrap();

    fuse_main();

    crypto_demo().unwrap();
}

use std::fs::File;
use std::io;
use std::io::prelude::*;

struct vm_metadata {
    pid: u32,
    imageid: u32,
    network: String,
    status: String,
    resources: vm_resource,
}

struct vm_resource {
    cpu: u32,
    memory: u32,
    storage: u32,
}

fn snapshot_cutting() -> io::Result<()> {
    const FNAME: &str = "test.txt";
    const CHUNK_SIZE: usize = 2 * 1024 * 1024; // bytes read by every loop iteration.
    let mut limit: usize = (1024 * 1024) * 15; // How much should be actually read from the file..
    let mut f = File::open(FNAME)?;
    let mut buffer = [0; CHUNK_SIZE]; // buffer to contain the bytes.

    // read up to 15mb as the limit suggests..
    loop {
        if limit > 0 {
            // Not finished reading, you can parse or process data.
            let _n = f.read(&mut buffer[..])?;

            for bytes_index in 0..buffer.len() {
                // print!("{}", buffer[bytes_index] as char);
                //Operation here

                if bytes_index % CHUNK_SIZE == 0 {
                    println!("x is divisible by {}", CHUNK_SIZE);
                    let filename = format!("output{}.txt", bytes_index); //bug code
                    let mut file = File::create(filename)?;
                    file.write_all(&buffer)?;
                    file.sync_all()?;
                    // println!("Data written to {} successfully.", filename);
                }
            }
            limit -= CHUNK_SIZE;
        } else {
            // Finished reading..
            break;
        }
    }
    Ok(())
}

use std::fs::{self};
use std::io::{Read, Write};

fn snapshot_cutting2() -> std::io::Result<()> {
    let mut large_file = File::open("mem_file")?;

    // Create a directory to store the small files
    fs::create_dir_all("small_files")?;

    // Define the size of each chunk (in bytes)
    let chunk_size = 2 * 1024 * 1024; // 2 MiB

    // Buffer to store each chunk
    let mut buffer = vec![0; chunk_size];

    // Counter for naming small files
    let mut file_counter = 1;

    loop {
        // Read a chunk from the large file
        let bytes_read = large_file.read(&mut buffer)?;

        if bytes_read == 0 {
            // Reached end of file
            break;
        }

        // Write the chunk to a small file
        let mut small_file = File::create(format!("small_files/part_{}.txt", file_counter))?;
        small_file.write_all(&buffer[..bytes_read])?;

        // Increment file counter
        file_counter += 1;
    }

    Ok(())
}

fn blocks_restoring() -> std::io::Result<()> {
    // Create the original file for writing
    let mut original_file = File::create("mem_file_orig")?;

    // Iterate over the small files and concatenate their contents
    let mut file_counter = 1;
    loop {
        let small_file_path = format!("small_files/part_{}.txt", file_counter);
        if !small_file_exists(&small_file_path) {
            // No more small files, exit loop
            break;
        }

        // Open the small file for reading
        let mut small_file = File::open(small_file_path)?;

        // Read the contents of the small file into a buffer
        let mut buffer = Vec::new();
        small_file.read_to_end(&mut buffer)?;

        // Write the contents of the small file to the original file
        original_file.write_all(&buffer)?;

        // Increment file counter
        file_counter += 1;
    }

    Ok(())
}

fn small_file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

use md5;

fn compute_hash() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}

use std::collections::HashMap;

fn save_into_hashmap() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
}

use uuid::Uuid;

fn generate_uuid() -> Uuid {
    let uuid = uuid::Uuid::new_v4();
    // println!("{}", uuid);
    uuid
}

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

fn crypto_demo() -> Result<(), chacha20poly1305::Error> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
    println!("{:?}", ciphertext);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
    match String::from_utf8(plaintext) {
        Ok(s) => {
            println!("Converted String: {}", s);
        }
        Err(e) => {
            println!("Error converting to String: {}", e);
        }
    }
    // assert_eq!(&plaintext, b"plaintext message");
    Ok(())
}

use fuser::{FileType, Filesystem, MountOption, ReplyAttr, Request};
use std::ffi::OsStr;
use std::time::{Duration, SystemTime};

struct MyFS;

impl Filesystem for MyFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let attr = fuser::FileAttr {
            ino: 1,
            size: 4096000000,
            blksize: 4096000,
            padding: 0,
            blocks: 1,
            atime: SystemTime::now().into(),
            mtime: SystemTime::now().into(),
            ctime: SystemTime::now().into(),
            crtime: SystemTime::now().into(),
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
        };
        reply.attr(&Duration::new(0, 0), &attr);
    }

    // Implement other filesystem methods as needed
}

use clap::{Arg, ArgAction, Command};

fn fuse_main() {
    // let matches = Command::new("hello")
    //     .author("Christopher Berner")
    //     .arg(
    //         Arg::new("MOUNT_POINT")
    //             .required(true)
    //             .index(1)
    //             .help("Act as a client, and mount FUSE at given path"),
    //     )
    //     .arg(
    //         Arg::new("auto_unmount")
    //             .long("auto_unmount")
    //             .action(ArgAction::SetTrue)
    //             .help("Automatically unmount on process exit"),
    //     )
    //     .arg(
    //         Arg::new("allow-root")
    //             .long("allow-root")
    //             .action(ArgAction::SetTrue)
    //             .help("Allow root user to access filesystem"),
    //     )
    //     .get_matches();
    env_logger::init();
    let mountpoint = "/home/weikang/Documents/serverlessinterface/testfolder";
    let filesystem = MyFS;
    let mut options = vec![MountOption::RW, MountOption::FSName("hello".to_string())];

    // options.push(MountOption::AutoUnmount);

    // options.push(MountOption::AllowRoot);

    fuser::mount2(filesystem, mountpoint, &options).unwrap();
}
