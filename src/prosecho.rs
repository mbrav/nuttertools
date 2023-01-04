use clap::{arg, ArgAction, Args};

use crate::Error;

/// The drunk pro's alternative to echo
#[derive(Args)]
pub struct Options {
    /// Input text as a space separated argument
    /// or as a string
    #[arg(required=true, value_name="TEXT", action=ArgAction::Append)]
    text: Vec<String>,
    /// Specify whether to newline
    #[arg(short = 'n', long = "newline", action=ArgAction::SetTrue)]
    newline: bool,
}

/// # Errors
///
/// Will return `Err` string is empty
pub fn main(opts: &Options) -> Result<(), Error> {
    let t = opts.text.join(" ");
    let sep = if opts.newline { "\n" } else { "" };
    if t.is_empty() {
        Err(Error::ValueError)
    } else {
        print!("{t}{sep}");
        Ok(())
    }
}
