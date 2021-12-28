use crate::data::{Ciphertext, Key, Plaintext};

pub fn repeating_key_xor_encrypt(plaintext: &Plaintext, key: &Key) -> Ciphertext {
    Ciphertext(repeating_key_xor(&plaintext.0, key))
}

pub fn repeating_key_xor_decrypt(ciphertext: &Ciphertext, key: &Key) -> Plaintext {
    Plaintext(repeating_key_xor(&ciphertext.0, key))
}

fn repeating_key_xor(input: &[u8], key: &Key) -> Vec<u8> {
    input
        .iter()
        .zip(key.0.iter().cycle())
        .map(|(l, r)| l ^ r)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_encrypt() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let plaintext = Plaintext::from_str(input).unwrap();
        let expected_ciphertext_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = Key::from_str("ICE").unwrap();
        let cipher_text_hex = hex::encode(repeating_key_xor_encrypt(&plaintext, &key).0);
        assert_eq!(expected_ciphertext_hex, cipher_text_hex);
    }
}
