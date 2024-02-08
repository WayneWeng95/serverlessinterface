fn main() {
    println!("Hello, world!");
    println!("{}", generate_uuid());

    snapshot_cutting().unwrap();

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
    const FNAME: &str = "testFile.txt";
    const CHUNK_SIZE: usize = 2*1024*1024; // bytes read by every loop iteration.
    let mut limit: usize = (1024 * 1024) * 4; // How much should be actually read from the file..
    let mut f = File::open(FNAME)?;
    let mut buffer = [0; CHUNK_SIZE]; // buffer to contain the bytes.

    // read up to 15mb as the limit suggests..
    loop {
        if limit > 0 {
            // Not finished reading, you can parse or process data.
            let _n = f.read(&mut buffer[..])?;

            for bytes_index in 0..buffer.len() {
                println!("{}", buffer[bytes_index] as char);
                //Operation here
            }
            limit -= CHUNK_SIZE;
        } else {
            // Finished reading..
            break;
        }
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
