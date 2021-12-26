#[cfg(debug_assertions)]
use crate::plot;

const MAX_KEY_SIZE: usize = 50; // The maximum key size we'll attempt
const CHUNKS_TO_COMPARE_AT_A_TIME: usize = 5;

#[allow(dead_code)]
pub fn find_key_length_candidates(ciphertext: &[u8], number_of_candidates: usize) -> Vec<usize> {
    let mut ks_scores: Vec<_> = (2..MAX_KEY_SIZE)
        .map(|ks| ciphertext.chunks(ks).take(CHUNKS_TO_COMPARE_AT_A_TIME))
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
    use crate::s1c5::repeating_key_xor;

    #[test]
    fn test_find_key_length() {
        let key = "{}/!!@#$axcss";
        dbg!(key.len());
        let plaintext = "This is my testing plaintext which is a very plain text but also a plaintext which is used to test my code.";
        let ciphertext = repeating_key_xor(plaintext, key);
        let number_of_candidates = 2;
        let found_candidates = find_key_length_candidates(&ciphertext, number_of_candidates);
        dbg!(&found_candidates);
        assert_eq!(found_candidates.len(), number_of_candidates);
        assert!(found_candidates.contains(&key.len()));
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            37,
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        )
    }
}
