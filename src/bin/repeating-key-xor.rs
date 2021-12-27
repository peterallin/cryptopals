use std::str::FromStr;

use anyhow::Result;
use cryptopals::data::{Key, Plaintext};
use cryptopals::s1c5::repeating_key_xor_encrypt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    plain_text_file: std::path::PathBuf,
    cipher_text_file: std::path::PathBuf,
    key: String,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let input = std::fs::read_to_string(options.plain_text_file)?;
    let plaintext = Plaintext::from_str(&input)?;
    let key = Key::from_str(&options.key)?;
    let cipher_text = repeating_key_xor_encrypt(&plaintext, &key);
    std::fs::write(options.cipher_text_file, cipher_text.0)?;
    Ok(())
}
