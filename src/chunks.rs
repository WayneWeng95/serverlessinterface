use std::fs::File;
use std::fs::{self};
use std::io;
use std::io::prelude::*;
use std::io::{Read, Write};

fn chunks_cutting() -> std::io::Result<()> {
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

fn small_file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn chunks_restoring() -> std::io::Result<()> {
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
