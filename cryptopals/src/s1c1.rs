use anyhow::Result;

#[allow(dead_code)]
pub fn hex_to_base64(hex: &str) -> Result<String> {
    let bin = hex_to_bin(hex)?;
    Ok(base64::encode(bin))
}

#[allow(dead_code)]
pub fn hex_to_base64_homebrew(hex: &str) -> Result<String> {
    Ok(hex_to_bin(hex)?
        .chunks(3)
        .map(chunk_to_6bits)
        .map(|x| {
            x.into_iter()
                .map(|c| c.map(|c| BASE64_CHARS[c]).unwrap_or('='))
        })
        .flatten()
        .collect())
}

fn chunk_to_6bits(chunk: &[u8]) -> Vec<Option<usize>> {
    return match *chunk {
        [x] => vec![Some(a(x)), Some(a_rest(x)), None, None],
        [x, y] => vec![Some(a(x)), Some(b(x, y)), Some(b_rest(y)), None],
        [x, y, z] => vec![Some(a(x)), Some(b(x, y)), Some(c(y, z)), Some(d(z))],
        _ => panic!("Invalid chunk length"),
    };

    fn a(x: u8) -> usize {
        ((x & 0b11111100) >> 2) as usize
    }

    fn a_rest(x: u8) -> usize {
        ((x & 0b00000011) << 4) as usize
    }

    fn b(x: u8, y: u8) -> usize {
        ((x & 0b00000011) << 4 | (y & 0b11110000) >> 4) as usize
    }

    fn b_rest(y: u8) -> usize {
        ((y & 0b00001111) << 2) as usize
    }

    fn c(y: u8, z: u8) -> usize {
        ((y & 0b00001111) << 2 | (z & 0b11000000) >> 6) as usize
    }

    fn d(z: u8) -> usize {
        (z & 0b00111111) as usize
    }
}

fn hex_to_bin(hex: &str) -> Result<Vec<u8>> {
    Ok(hex::decode(hex)?)
}

static BASE64_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        let input = "00010203";
        let expected = vec![0, 1, 2, 3];
        assert_eq!(expected, hex_to_bin(input).unwrap());
    }

    #[test]
    fn test_encode() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(expected.to_string(), hex_to_base64(input).unwrap());
    }

    #[test]
    fn test_encode_1_byte_short() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=";

        assert_eq!(expected.to_string(), hex_to_base64(input).unwrap());
    }

    #[test]
    fn test_encode_2_bytes_short() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hybw==";

        assert_eq!(expected.to_string(), hex_to_base64(input).unwrap());
    }

    #[test]
    fn test_encode_homebrew() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(expected.to_string(), hex_to_base64_homebrew(input).unwrap());
    }

    #[test]
    fn test_encode_homebrew_1_byte_short() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=";

        assert_eq!(expected.to_string(), hex_to_base64_homebrew(input).unwrap());
    }

    #[test]
    fn test_encode_homebrew_2_bytes_short() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hybw==";

        assert_eq!(expected.to_string(), hex_to_base64_homebrew(input).unwrap());
    }
}
