use aes::cipher::BlockEncryptMut;
use aes::NewBlockCipher;
use anyhow::Result;
use rand::Rng;

use crate::data::{Ciphertext, Iv, Key, Plaintext};
use crate::set1::challenge8::count_repetitions;
use crate::set2::challenge10::aes128_cbc_encrypt;
use crate::set2::challenge9::pkcs7_pad;

pub fn aes128_ecb_encrypt(plaintext: &Plaintext, key: &Key) -> Result<Ciphertext> {
    let plaintext = pkcs7_pad(&plaintext, 16);
    let mut cipher = aes::Aes128::new_from_slice(&key.0).expect("Usable key string");
    let ciphertext = plaintext
        .0
        .chunks(16)
        .map(|chunk| {
            let mut chunk = *aes::cipher::generic_array::GenericArray::from_slice(&chunk);
            cipher.encrypt_block_mut(&mut chunk);
            chunk.to_vec()
        })
        .flatten()
        .collect::<Vec<_>>();
    Ok(Ciphertext(ciphertext))
}
#[derive(Debug, Eq, PartialEq)]
pub enum EncryptionMode {
    ECB,
    CBC,
}

pub fn encryption_oracle(
    input: &Plaintext,
    encryption_mode: Option<EncryptionMode>,
) -> Result<Ciphertext> {
    let prefix_len = rand::thread_rng().gen_range(5..=10);
    let prefix: Vec<u8> = (0..prefix_len).map(|_| rand::thread_rng().gen()).collect();
    let postfix_len = rand::thread_rng().gen_range(5..=10);
    let postfix: Vec<u8> = (0..postfix_len).map(|_| rand::thread_rng().gen()).collect();
    let fixed_plaintext: Vec<_> = prefix
        .into_iter()
        .chain(input.0.clone().into_iter())
        .chain(postfix.into_iter())
        .collect();
    let key: Vec<_> = (0..16).map(|_| rand::random()).collect();
    let encryption_mode = encryption_mode.unwrap_or_else(|| {
        if rand::random::<bool>() {
            EncryptionMode::CBC
        } else {
            EncryptionMode::ECB
        }
    });
    match encryption_mode {
        EncryptionMode::CBC => {
            println!("Chose CBC");
            let iv: Vec<_> = (0..16).map(|_| rand::random()).collect();
            aes128_cbc_encrypt(&Plaintext(fixed_plaintext), &Key(key), Iv(iv))
        }
        EncryptionMode::ECB => {
            println!("Chose ECB");
            aes128_ecb_encrypt(&Plaintext(fixed_plaintext), &Key(key))
        }
    }
}

pub fn guess_encryption_mode(input: &Ciphertext) -> EncryptionMode {
    if count_repetitions(16, &input.0)
        .into_iter()
        .all(|(_data, count)| count == 1)
    {
        EncryptionMode::CBC
    } else {
        EncryptionMode::ECB
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::challenge7::aes128_ecb_decrypt;
    use std::str::FromStr;

    #[test]
    fn test_random() {
        let plaintext = Plaintext::from_str(&"Yellow submarine".repeat(3)).unwrap();
        (1..100).for_each(|_| {
            let ciphertext = encryption_oracle(&plaintext, Some(EncryptionMode::ECB)).unwrap();
            let guess = guess_encryption_mode(&ciphertext);
            assert_eq!(EncryptionMode::ECB, guess);
        });
        (1..100).for_each(|_| {
            let ciphertext = encryption_oracle(&plaintext, Some(EncryptionMode::CBC)).unwrap();
            let guess = guess_encryption_mode(&ciphertext);
            assert_eq!(EncryptionMode::CBC, guess);
        });
    }

    #[test]
    fn test_aes128_ecb_encrypt() {
        let plaintext = Plaintext::from_str("This is my testing plaintext which is a very plain text but also a plaintext which is used to test my code.").unwrap();
        let key = Key::from_str("0123456789abcdef").unwrap();
        let ciphertext = aes128_ecb_encrypt(&plaintext, &key).unwrap();
        let key = Key::from_str("0123456789abcdef").unwrap();
        let new_plaintext = aes128_ecb_decrypt(&ciphertext, &key).unwrap();
        assert_eq!(plaintext.0, new_plaintext.0);
    }
}
