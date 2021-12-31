use crate::data::{Ciphertext, Key, Plaintext};
use aes::Aes128;
use anyhow::{Context, Result};
use block_modes::block_padding::{Pkcs7};
use block_modes::{BlockMode, Ecb};

pub type Aes128Ecb = Ecb<Aes128, Pkcs7>;

pub fn aes128_ecb_decrypt(ciphertext: &Ciphertext, key: &Key) -> Result<Plaintext> {
    let cipher = Aes128Ecb::new_from_slices(&key.0, &[]).context("Creating cipher")?;
    let plaintext = cipher.decrypt_vec(&ciphertext.0).context("Decrypting an ECB block")?;
    Ok(Plaintext(plaintext))
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_aes128_ecb_decrypt() {
        let ciphertext: String = include_str!("7.txt")
            .chars()
            .filter(|c| *c != '\n')
            .collect();
        let ciphertext = Ciphertext(base64::decode(ciphertext).unwrap());
        let key = Key::from_str("YELLOW SUBMARINE").unwrap();
        let plaintext = aes128_ecb_decrypt(&ciphertext, &key).unwrap();
        let plaintext = String::from_utf8_lossy(&plaintext.0);
        assert!(plaintext.contains("Play that funky music"));
    }
}
