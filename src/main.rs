fn main() {
    println!("Hello, world!");
    println!("{}", generate_uuid());

    snapshot_cutting2().unwrap();

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
