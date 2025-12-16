

use aes_gcm::{Aes256Gcm, Key, Nonce}; // Import what you need
use aes_gcm::aead::{Aead, KeyInit};

// Type definition for easy error handling
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// 1. Helper: Password string se 32-byte key banana
fn derive_key(password: &str) -> Key<Aes256Gcm> {
    // Logic: Use SHA256 hashing here
    todo!("Implement hashing logic")
}

// 2. Encrypt Function
pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>> {
    // Logic: 
    // - Derive Key
    // - Generate Random Nonce (12 bytes)
    // - Encrypt data
    // - Return vec![Nonce + Ciphertext]
    todo!("Implement encryption")
}

// 3. Decrypt Function
pub fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>> {
    // Logic:
    // - Extract Nonce (first 12 bytes)
    // - Extract Ciphertext (rest)
    // - Derive Key
    // - Decrypt and return
    todo!("Implement decryption")
}

