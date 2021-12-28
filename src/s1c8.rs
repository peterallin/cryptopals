use crate::data::Ciphertext;
use std::collections::HashMap;

#[allow(dead_code)]
fn find_ciphertext_with_most_repetitions(
    block_size: usize,
    ciphertexts: &[Ciphertext],
) -> &Ciphertext {
    ciphertexts
        .iter()
        .map(|ct| (ct, count_repetitions(block_size, &ct.0)))
        .map(|(ct, r)| (ct, r.values().sum::<usize>() * 100 / r.len()))
        .max_by_key(|(_ct, x)| *x)
        .unwrap()
        .0
}

fn count_repetitions(block_size: usize, data: &[u8]) -> HashMap<&[u8], usize> {
    let mut result = HashMap::new();
    for c in data.chunks(block_size) {
        *result.entry(c).or_insert(0) += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_ciphertext_with_most_repetitions() {
        let input = include_str!("8.txt");
        let inputs = input
            .lines()
            .map(|x| hex::decode(x))
            .filter_map(Result::ok)
            .map(|i| Ciphertext(i));
        let inputs = inputs.collect::<Vec<Ciphertext>>();
        let ecb_encrypted = find_ciphertext_with_most_repetitions(16, &inputs);
        assert_eq!(ecb_encrypted.0, inputs[132].0); // 132 has been checked against other peoples solutions.
    }
}
