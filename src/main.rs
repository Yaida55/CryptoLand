mod ciphers;

use ciphers::caesar;
use ciphers::stream::RC4;

fn main() {
    // カエサル暗号
    let plaintext = "Hello, World!";
    let shift = 3;
    
    let encrypted = caesar::caesar_encrypt(plaintext, shift);
    println!("暗号化: {}", encrypted);
    
    let decrypted = caesar::caesar_decrypt(&encrypted, shift);
    println!("復号化: {}", decrypted);

    // ストリーム暗号
    let key = b"Secret Key";
    let mut rc4 = RC4::new(key);
    
    let mut message = b"Hello, World!".to_vec();
    println!("Original: {:?}", String::from_utf8_lossy(&message));
    
    rc4.encrypt(&mut message);
    println!("Encrypted: {:?}", message);
    
    let mut rc4 = RC4::new(key);
    rc4.encrypt(&mut message);
    println!("Decrypted: {:?}", String::from_utf8_lossy(&message));
}
