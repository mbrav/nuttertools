use clap::{arg, Args};
use sha1::{Digest, Sha1};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

/// Your trusted SHA1 cracking dealer on the block
#[derive(Args)]
pub struct Options {
    /// Hash (40 symbols)
    #[arg(required = true, value_name = "SHA1")]
    pub hash: String,

    /// File Path.
    /// Example: https://github.com/danielmiessler/SecLists/raw/master/Passwords/Common-Credentials/10-million-password-list-top-1000000.txt
    #[arg(short = 'f', long = "file")]
    pub file: String,
}

/// # Errors
///
/// Will return `Err` if SHA1 hash is not valid
pub fn main(opts: &Options) -> Result<(), Box<dyn Error>> {
    if opts.hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 hash is not valid".into());
    }

    let file = File::open(&opts.file)?;
    let reader = BufReader::new(file);

    for (l, line) in reader.lines().enumerate() {
        let line = line?;
        let password = line.trim();

        if opts.hash == format!("{:x}", Sha1::digest(password.as_bytes())) {
            println!("Password found: {password}");
            println!("Line Number: {}", l + 1);
            return Ok(());
        }
    }

    println!("Password not found in wordlist");
    Ok(())
}
