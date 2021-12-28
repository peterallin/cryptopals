#[cfg(debug_assertions)]
use crate::plot;

use crate::data::{Ciphertext, Key, Plaintext};
use crate::set1::challenge3::break_single_byte_xor;
use crate::set1::challenge5::repeating_key_xor_decrypt;

const MAX_KEY_SIZE: usize = 60; // The maximum key size we'll attempt
const CHUNKS_TO_COMPARE_AT_A_TIME: usize = 10;

pub fn break_repeating_key_xor(ciphertext: &Ciphertext) -> (Key, Plaintext) {
    let key_length_candidates = dbg!(find_key_length_candidates(ciphertext, 10));
    let chosen_key_length = dbg!(key_length_candidates[0]);
    let transposed_blocks = transpose_blocks(&ciphertext.0, chosen_key_length);
    let key: Key = transposed_blocks
        .iter()
        .map(|tb| break_single_byte_xor(tb))
        .map(|(key, _plain)| key)
        .collect();
    let plaintext = repeating_key_xor_decrypt(ciphertext, &key);
    (key, plaintext)
}

fn find_key_length_candidates(ciphertext: &Ciphertext, number_of_candidates: usize) -> Vec<usize> {
    let mut ks_scores: Vec<_> = (2..MAX_KEY_SIZE)
        .map(|ks| ciphertext.0.chunks(ks).take(CHUNKS_TO_COMPARE_AT_A_TIME))
        .map(|c| {
            let v: Vec<_> = c.collect();
            let len = v[0].len();
            let hamming_values = multi_hamming(&v);
            if hamming_values.is_empty() {
                None
            } else {
                let mh: usize = hamming_values.iter().sum::<usize>() / hamming_values.len();
                Some((len, mh))
            }
        })
        .flatten()
        .collect();

    #[cfg(debug_assertions)]
    plot::simple_xy("score vs keysize", "ks_scores.png", &ks_scores);

    ks_scores.sort_by_key(|(_ks, score)| *score);
    ks_scores
        .iter()
        .take(number_of_candidates)
        .map(|x| x.0)
        .collect()
}

fn transpose_blocks(ciphertext: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::<Vec<u8>>::new();
    result.resize(keysize, vec![]);
    for block in ciphertext.chunks(keysize) {
        for (index, c) in block.iter().enumerate() {
            result[index].push(*c);
        }
    }
    result
}

fn multi_hamming(data: &[&[u8]]) -> Vec<usize> {
    let len = data[0].len();
    let mut data = data.iter();
    let base = data.next().unwrap();
    data.filter(|d| d.len() == len)
        .map(|d| {
            d.len();
            base.len();
            hamming_distance(base, d) * 100 / len
        })
        .collect()
}

fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2.iter())
        .map(|(a, b)| differing_bits(*a, *b))
        .sum()
}

fn differing_bits(a: u8, b: u8) -> usize {
    (0..7)
        .map(|x| (1 << x) & (a ^ b))
        .filter(|x| *x != 0)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::challenge5::repeating_key_xor_encrypt;
    use std::str::FromStr;

    #[test]
    fn test_find_key_length() {
        let key = Key::from_str("{}/!!@#$axcss").unwrap();
        let plaintext = Plaintext::from_str("This is my testing plaintext which is a very plain text but also a plaintext which is used to test my code.").unwrap();
        let ciphertext = repeating_key_xor_encrypt(&plaintext, &key);
        let number_of_candidates = 2;
        let found_candidates = find_key_length_candidates(&ciphertext, number_of_candidates);
        dbg!(&found_candidates);
        assert_eq!(found_candidates.len(), number_of_candidates);
        assert!(found_candidates.contains(&key.0.len()));
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            37,
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        )
    }

    #[test]
    fn test_transpose_blocks() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let transposed = transpose_blocks(&input, 3);
        assert_eq!(transposed[0], vec![1, 4, 7]);
        assert_eq!(transposed[1], vec![2, 5, 8]);
        assert_eq!(transposed[2], vec![3, 6, 9]);
    }

    #[test]
    fn test_transpose_blocks_length_doesnt_match() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let transposed = transpose_blocks(&input, 4);
        assert_eq!(transposed[0], vec![1, 5, 9]);
        assert_eq!(transposed[1], vec![2, 6]);
        assert_eq!(transposed[2], vec![3, 7]);
        assert_eq!(transposed[3], vec![4, 8]);
    }

    #[test]
    fn test_break_repeating_key_xor() {
        let ciphertext_base64: String = include_str!("6.txt")
            .chars()
            .filter(|c| *c != '\n')
            .collect();
        let ciphertext = Ciphertext(base64::decode(ciphertext_base64).unwrap());
        let (key, _plaintext) = break_repeating_key_xor(&ciphertext);
        assert_eq!("Terminator X: Bring the noise", key.to_string());
    }
}
