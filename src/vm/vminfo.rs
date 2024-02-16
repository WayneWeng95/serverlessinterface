struct vm_metadata {
    pid: u32,
    imageid: u32,
    network: String,
    status: String,
    resources: vm_resource,
    socket_path: String,
}

struct vm_resource {
    cpu: u32,
    memory: u32,
    storage: u32,
}


use std::collections::HashMap;

fn save_into_hashmap() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
}

use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    let uuid = uuid::Uuid::new_v4();
    // println!("{}", uuid);
    uuid
}