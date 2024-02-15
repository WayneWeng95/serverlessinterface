use md5;

fn compute_hash() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub fn crypto_demo() -> Result<(), chacha20poly1305::Error> {
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
