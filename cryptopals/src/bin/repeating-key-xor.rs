use anyhow::Result;
use structopt::StructOpt;
use cryptopals::s1c5::repeating_key_xor;

#[derive(StructOpt)]
struct Options {
    plain_text_file: std::path::PathBuf,
    cipher_text_file: std::path::PathBuf,
    key: String,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let plain_text = std::fs::read_to_string(options.plain_text_file)?;
    let cipher_text = repeating_key_xor(&plain_text, &options.key);
    std::fs::write(options.cipher_text_file, cipher_text)?;
    Ok(())
}
