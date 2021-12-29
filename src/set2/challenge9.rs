use crate::data::Plaintext;

pub fn pkcs7_pad(input: &Plaintext, block_size: usize) -> Plaintext {
    let mut data = input.0.clone();
    let number_of_blocks = dbg!((data.len() + block_size - 1) / block_size);
    let bytes_to_add = dbg!(number_of_blocks * block_size - data.len());
    data.resize(data.len() + bytes_to_add, bytes_to_add as u8);
    Plaintext(data)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_pkcs7_pad_1blocks_of_20bytes_16bytes_input() {
        let plaintext = "YELLOW SUBMARINE";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 20);
        assert_eq!(20, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..16]);
        assert_eq!(vec![4, 4, 4, 4], plaintext_padded.0[16..20]);
    }

    #[test]
    fn test_pkcs7_pad_2blocks_of_10bytes_16bytes_input() {
        let plaintext = "YELLOW SUBMARINE";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 10);
        assert_eq!(20, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..16]);
        assert_eq!(vec![4, 4, 4, 4], plaintext_padded.0[16..20]);
    }

    #[test]
    fn test_pkcs7_pad_1block_of_20bytes_19bytes_input() {
        let plaintext = "YELLOW SUBMARINE123";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 20);
        assert_eq!(20, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..19]);
        assert_eq!(vec![1], plaintext_padded.0[19..20]);
    }

    #[test]
    fn test_pkcs7_pad_2blocks_of_10bytes_19bytes_input() {
        let plaintext = "YELLOW SUBMARINE123";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 10);
        assert_eq!(20, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..19]);
        assert_eq!(vec![1], plaintext_padded.0[19..20]);
    }

    #[test]
    fn test_pkcs7_pad_2blocks_of_11bytes_16bytes_input() {
        let plaintext = "YELLOW SUBMARINE";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 11);
        assert_eq!(22, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..16]);
        assert_eq!(vec![6, 6, 6, 6, 6, 6], plaintext_padded.0[16..22]);
    }

    #[test]
    fn test_pkcs7_pad_no_pad() {
        let plaintext = "YELLOW SUBMARINE";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 16);
        assert_eq!(16, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0);
    }

    #[test]
    fn test_pkcs7_pad_full_pad() {
        let plaintext = "YELLOW SUBMARINES";
        let plaintext = Plaintext::from_str(plaintext).unwrap();
        let plaintext_padded = pkcs7_pad(&plaintext, 16);
        assert_eq!(32, plaintext_padded.0.len());
        assert_eq!(plaintext.0, plaintext_padded.0[0..17]);
        assert_eq!(
            vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15],
            plaintext_padded.0[17..32]
        );
    }
}
