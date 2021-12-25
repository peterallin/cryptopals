
pub fn repeating_key_xor(plain_text: &str, key: &str) -> Vec<u8> {
    plain_text
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(l, r)| l ^ r)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encrypt() {
        let plain_text =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected_cipher_text = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = "ICE";
        let cipher_text = hex::encode(repeating_key_xor(plain_text, key));
        assert_eq!(expected_cipher_text, cipher_text);
    }
}