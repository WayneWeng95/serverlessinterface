
fn main() {
    println!("Hello, world!");
}

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn snapshot_cutting() -> io::Result<()> {
    const FNAME: &str = "LargeFile.txt";
    const CHUNK_SIZE: usize = 1024; // bytes read by every loop iteration.
    let mut limit: usize = (1024 * 1024) * 15; // How much should be actually read from the file..
    let mut f = File::open(FNAME)?;
    let mut buffer = [0; CHUNK_SIZE]; // buffer to contain the bytes.

    // read up to 15mb as the limit suggests..
    loop {
        if limit > 0 {
            // Not finished reading, you can parse or process data.
            let _n = f.read(&mut buffer[..])?;

            for bytes_index in 0..buffer.len() {
                print!("{}", buffer[bytes_index] as char);
            }
            limit -= CHUNK_SIZE;
        } else {
            // Finished reading..
            break;
        }
    }
    Ok(())
}
