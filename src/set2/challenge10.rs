use aes::cipher::{BlockDecryptMut, BlockEncryptMut};
use aes::NewBlockCipher;
use anyhow::Result;

use crate::data::{Ciphertext, Iv, Key, Plaintext};
use crate::set1::challenge2::fixed_xor;
use crate::set2::challenge9::pkcs7_pad;

pub fn aes128_cbc_decrypt(ciphertext: &Ciphertext, key: &Key, mut iv: Iv) -> Result<Plaintext> {
    let mut plaintext: Vec<u8> = vec![];
    let mut cipher = aes::Aes128::new_from_slice(&key.0).unwrap();
    for ciphertext_chunk in ciphertext.0.chunks(16) {
        let next_iv = ciphertext_chunk.to_vec();
        let mut chunk = *aes::cipher::generic_array::GenericArray::from_slice(ciphertext_chunk);
        cipher.decrypt_block_mut(&mut chunk);
        let plaintext_chunk = fixed_xor(&chunk, &iv.0);
        iv.0 = next_iv;
        plaintext.extend(&plaintext_chunk);
    }
    Ok(Plaintext(plaintext)) // TODO: de-pad
}


pub fn aes128_cbc_encrypt(plaintext: &Plaintext, key: &Key, mut iv: Iv) -> Result<Ciphertext> {
    let plaintext = pkcs7_pad(&plaintext, 16);
    let mut ciphertext : Vec<u8> = vec![];
    let mut cipher = aes::Aes128::new_from_slice(&key.0).unwrap();
    for plaintext_chunk in plaintext.0.chunks(16) {
        let xored_chunk  = fixed_xor(plaintext_chunk, &iv.0);
        let mut chunk = *aes::cipher::generic_array::GenericArray::from_slice(&xored_chunk);
        cipher.encrypt_block_mut(&mut chunk);
        iv.0 = chunk.to_vec();
        ciphertext.extend(&chunk);
    }

    Ok(Ciphertext(ciphertext))
}


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_aes128_cbc_decrypt() {
        let ciphertext: String = include_str!("10.txt")
            .chars()
            .filter(|c| *c != '\n')
            .collect();
        let ciphertext = Ciphertext(base64::decode(ciphertext).unwrap());
        let key = Key::from_str("YELLOW SUBMARINE").unwrap();
        let iv = Iv(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let plaintext = aes128_cbc_decrypt(&ciphertext, &key, iv).unwrap();
        let plaintext = dbg!(String::from_utf8(plaintext.0).unwrap());
        assert!(plaintext.contains("Play that funky music, white boy"));
    }

    #[test]
    fn test_aes128_cbc_encrypt() {
        let key = Key::from_str("0123456789abcdef").unwrap();
        let iv = Iv(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let plaintext = Plaintext::from_str("This is my testing plaintext which is a very plain text but also a plaintext which is used to test my code.").unwrap();
        let ciphertext = aes128_cbc_encrypt(&plaintext, &key, iv).unwrap();
        let iv = Iv(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let new_plaintext = aes128_cbc_decrypt(&ciphertext, &key, iv).unwrap();
        assert_eq!(pkcs7_pad(&plaintext, 16).0, new_plaintext.0);
    }
}
