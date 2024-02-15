use reqwest::blocking::Client;
use std::collections::HashMap;

pub fn set_boot_source() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Define the URL and request body
    let url = "http://localhost/boot-source";
    let body = r#"{
        "kernel_image_path": "/home/shared/images/vmlinux-5.10.198",
        "boot_args": "console=ttyS0 reboot=k panic=1 pci=off"
    }"#;

    // Create a hashmap for headers
    let mut headers = HashMap::new();
    headers.insert("Accept", "application/json");
    headers.insert("Content-Type", "application/json");

    // Make the request
    let response = client
        .put(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    // Print the response status code and body
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n{}", response.text()?);

    Ok(())
}
