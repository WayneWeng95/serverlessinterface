use super::vminfo::*;
use crate::api;
use crate::vm;
use std::io;

pub async fn set_up_vm(iplibrary: & mut IpLibrary) -> io::Result<()> {
    let vmsetup = vm::vminfo::VmSetUp::default_test();//VM setup function here, for later the data need to be import

    match api::firecrackerapi::initialize_vm(&vmsetup, iplibrary).await {
        Ok(_) => {
            println!("VM configured successfully");
            api::firecrackerapi::instance_control(&vmsetup.socket_path, VmStatus::Running).await?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}


