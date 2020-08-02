use base64;
use fernet;

///加密字符串
fn encrypt(plaintext: String, with_key: String) -> String {
    let cipher_suite = fernet::Fernet::new(&with_key).unwrap();
    let ciphertext = cipher_suite.encrypt(plaintext.as_bytes());
    println!("{}", ciphertext);
    let b64 = base64::encode(ciphertext);
    b64
}

///解密字符串
fn decrypt(encrypted_text: String, with_key: String) -> String {
    let cipher_suite = fernet::Fernet::new(&with_key).unwrap();
    let ciphertext = base64::decode(encrypted_text.as_bytes()).unwrap();
    let origin_text = cipher_suite
        .decrypt(std::str::from_utf8(&ciphertext).unwrap())
        .unwrap();
    String::from_utf8(origin_text).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_for_encrypt_and_decrypt() {
        let plaintext = String::from("abcdefg");
        let key = String::from("HsSaJEi9xNV1tqM3z2Th0kWwY5ys64WTE5Li7YL0RLg=");
        let encrypted = encrypt(plaintext.clone(), key.clone());
        let decrypted = decrypt(encrypted, key.clone());
        assert_eq!(decrypted, plaintext)
    }

    #[test]
    pub fn test_for_decrypt() {
        let plaintext = String::from("abcdefg");
        let key = String::from("HsSaJEi9xNV1tqM3z2Th0kWwY5ys64WTE5Li7YL0RLg=");
        let encrypted = String::from("Z0FBQUFBQmZKaGxyaHVnQ2pJMWNkc0o2RG9mbGpuTWlQNXBadTFGbDlWQzdJLXQ4WDVnU3EzREdMZkFCWnAzREtVTXQzQ2pmdW1VSldpdmQwSl9vZGtobndNOE1VR1V1WHc9PQ==");
        assert_eq!(plaintext, decrypt(encrypted, key))
    }
}
