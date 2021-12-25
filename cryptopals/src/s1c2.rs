use anyhow::{Context, Result};

#[allow(dead_code)]
pub fn fixed_xor(l: &[u8], r: &[u8]) -> Vec<u8> {
    if l.len() != r.len() {
        panic!("The inputs to fixed xor should have the same size");
    }

    l.iter()
        .zip(r.iter())
        .map(|(l, r)| l ^ r)
        .collect()
}

#[allow(dead_code)]
pub fn fixed_xor_hex(l: &str, r: &str) -> Result<String> {
    let l = hex::decode(l).context("Decoding left hex string")?;
    let r = hex::decode(r).context("Decoding right hex string")?;
    let result = fixed_xor(&l, &r);
    Ok(hex::encode(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(expected, fixed_xor_hex(input1, input2).unwrap());
    }
}
