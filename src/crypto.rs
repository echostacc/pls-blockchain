use openssl::symm::{Cipher, decrypt, encrypt};
use rand::{Rng, thread_rng};

pub fn hash(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    result.into()
}

pub fn hash_hex(data: &[u8]) -> String {
    let hash_bytes = hash(data);
    hash_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

pub fn encrypt_aes256_cbc(
    data: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut iv = [0u8; 16];
    thread_rng().fill(&mut iv);

    let cipher = Cipher::aes_256_cbc();
    let ciphertext = encrypt(cipher, key, Some(&iv), data)?;

    let mut result = iv.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

pub fn decrypt_aes256_cbc(
    encrypted_data: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if encrypted_data.len() < 16 {
        return Err("Invalid encrypted data length".into());
    }

    let (iv, ciphertext) = encrypted_data.split_at(16);

    let cipher = Cipher::aes_256_cbc();
    let decrypted = decrypt(cipher, key, Some(iv), ciphertext)?;

    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let data = b"Hello, World!";

        let encrypted = encrypt_aes256_cbc(data, &key).unwrap();
        let decrypted = decrypt_aes256_cbc(&encrypted, &key).unwrap();

        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_hash() {
        let data = b"Hello, World!";
        let hash_result = hash(data);

        let expected: [u8; 32] = [
            0xdf, 0xfd, 0x60, 0x21, 0xbb, 0x2b, 0xd5, 0xb0, 0xaf, 0x67, 0x62, 0x90, 0x80, 0x9e,
            0xc3, 0xa5, 0x31, 0x91, 0xdd, 0x81, 0xc7, 0xf7, 0x0a, 0x4b, 0x28, 0x68, 0x8a, 0x36,
            0x21, 0x82, 0x98, 0x6f,
        ];
        assert_eq!(hash_result, expected);
    }

    #[test]
    fn test_hash_hex() {
        let data = b"Hello, World!";
        let hash_result = hash_hex(data);

        let expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
        assert_eq!(hash_result, expected);
    }
}
