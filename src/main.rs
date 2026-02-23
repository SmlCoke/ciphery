use ciphery::caesar::{encrypt, decrypt};

fn main() {
    let string = "Hello, World! 123 你好😅";

    let shift = 5;
    let encrypted = encrypt(string, shift);
    println!("Encrypted: {}", encrypted);
    let decrypted = decrypt(&encrypted, shift);
    println!("Decrypted: {}", decrypted);

}
