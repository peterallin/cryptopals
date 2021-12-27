use crate::s1c3::{break_single_byte_xor, rate};

#[allow(dead_code)]
pub fn detect_single_byte_xor(candidates: &[Vec<u8>]) -> String {
    candidates
        .iter()
        .map(|x| break_single_byte_xor(x))
        .map(|(_key, plain)| (rate(&plain), plain))
        .max_by_key(|x| x.0)
        .unwrap()
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_single_byte_xor() {
        let data: Vec<Vec<u8>> = include_str!("4.txt")
            .lines()
            .map(hex::decode)
            .map(|x| x.unwrap())
            .collect();
        let plaintext = detect_single_byte_xor(&data);
        assert_eq!("Now that the party is jumping\n".to_string(), plaintext);
    }
}
