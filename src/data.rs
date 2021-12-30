use std::convert::Infallible;

pub struct Key(pub Vec<u8>);

pub struct Iv(pub Vec<u8>);

#[derive(Debug)]
pub struct Plaintext(pub Vec<u8>);

#[derive(Debug)]
pub struct Ciphertext(pub Vec<u8>);

impl std::string::ToString for Key {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl std::string::ToString for Plaintext {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl std::string::ToString for Ciphertext {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl std::str::FromStr for Key {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key(s.as_bytes().to_owned()))
    }
}

impl std::str::FromStr for Plaintext {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Plaintext(s.as_bytes().to_owned()))
    }
}

impl std::iter::FromIterator<u8> for Ciphertext {
    fn from_iter<I: IntoIterator<Item = u8>>(i: I) -> Self {
        Ciphertext(i.into_iter().collect())
    }
}

impl std::iter::FromIterator<u8> for Key {
    fn from_iter<I: IntoIterator<Item = u8>>(i: I) -> Self {
        Key(i.into_iter().collect())
    }
}
