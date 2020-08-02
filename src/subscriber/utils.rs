use crate::config::Config;
use base64;
use fernet;
use mailgun_v3::email::{send_email, EmailAddress, Message, MessageBody};
use mailgun_v3::Credentials;

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

// TODO: aync
///发送邮件
pub fn send_an_email(
    subject: String,
    message: String,
    html_message: String,
    recipient: String,
) -> bool {
    let config = Config::from_env().unwrap();
    let key = config.mailgun.api_key;
    let domain = config.mailgun.domain;
    let creds = Credentials::new(&key, &domain);
    let recipient = EmailAddress::address(&recipient);
    let message = Message {
        to: vec![recipient],
        subject: subject,
        body: MessageBody::HtmlAndText(html_message, message),
        ..Default::default()
    };
    let sender = EmailAddress::name_address("CodeDays", "mail@codedays.app");

    let res = send_email(&creds, &sender, message);
    res.is_ok()
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

    #[test]
    pub fn test_for_send_mail() {
        let ok = send_an_email(
            String::from("Test"),
            String::from("Test"),
            String::from("Test"),
            String::from("qwh005007@gmail.com"),
        );
        assert_eq!(ok, true)
    }
}
