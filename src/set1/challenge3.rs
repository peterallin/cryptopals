#[allow(dead_code)]
pub fn break_single_byte_xor(ciphertext_bin: &[u8]) -> (u8, String) {
    (0..255)
        .map(|key| (key, try_decode(key, ciphertext_bin)))
        .map(|(key, plain)| (rate(&plain), key, plain))
        .max_by_key(|(score, _key, _plain)| *score)
        .map(|(_score, key, plain)| (key, plain))
        .unwrap()
}

pub fn rate(message: &str) -> isize {
    message.chars().map(rate_char).sum::<isize>() * 100 / message.len() as isize
}

fn try_decode(key: u8, ciphertext: &[u8]) -> String {
    let x: Vec<_> = ciphertext.iter().map(|c| c ^ key).collect();
    String::from_utf8_lossy(&x).to_string()
}

fn rate_char(c: char) -> isize {
    if is_vowel(c) || c == ' ' {
        25
    } else if is_alpha(c) {
        10
    } else if is_upper_alpha(c) {
        1
    } else {
        -50
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c)
}

fn is_upper_alpha(c: char) -> bool {
    ('A'..='Z').contains(&c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_break_single_byte_xor() {
        let ciphertext_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext_bin = hex::decode(ciphertext_hex).unwrap();
        let (_key, plaintext) = break_single_byte_xor(&ciphertext_bin);
        assert_eq!("Cooking MC's like a pound of bacon", plaintext);
    }
}
