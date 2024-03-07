use super::vminfo::*;

pub fn set_vmnetwork(seeds: i32, mac: &str) -> VMnetowrk {
    let (remainder, quotient) = calculate_mod_and_divide(seeds);

    // register_network(seeds, remainder, quotient);    //System level registration

    let netowrk = VMnetowrk::new(
        format!("172.16.{}.{}", quotient, remainder),
        format!("net{}", seeds),
        mac.to_string(),
        format!("tap{}", seeds),
    );

    // println!("VM network: {:#?}", netowrk);
    netowrk
}

fn calculate_mod_and_divide(number: i32) -> (i32, i32) {
    // Calculate the remainder (modulus) when dividing by 255
    let remainder = number % 255;

    // Calculate the quotient when dividing by 255
    let quotient = number / 255;

    // Return the remainder and quotient as a tuple
    (remainder, quotient)
}
