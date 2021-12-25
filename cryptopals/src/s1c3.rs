#[allow(dead_code)]
pub fn break_single_byte_xor(ciphertext_bin: &[u8]) -> String {
    (0..255)
        .map(|key| try_decode(key, ciphertext_bin))
        .map(|plain| (rate(&plain), plain))
        .max_by_key(|x| x.0)
        .unwrap()
        .1
}

pub fn rate(message: &str) -> usize {
    message.matches(is_vowel).count() * 100 / message.len()
}

fn try_decode(key: u8, ciphertext: &[u8]) -> String {
    let x: Vec<_> = ciphertext.iter().map(|c| c ^ key).collect();
    String::from_utf8_lossy(&x).to_string()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_break_single_byte_xor() {
        let ciphertext_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext_bin = hex::decode(ciphertext_hex).unwrap();
        let plaintext = break_single_byte_xor(&ciphertext_bin);
        assert_eq!("Cooking MC's like a pound of bacon", plaintext);
    }
}
