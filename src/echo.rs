use clap::{arg, ArgAction, Args};

#[derive(Debug)]
pub enum Error {
    EmptyString,
}

#[derive(Args)]
pub struct Options {
    /// Input text as space separated argument
    /// or as arguments
    #[arg(required=true, value_name="TEXT", action=ArgAction::Append)]
    text: Vec<String>,
    /// Specify whether to newline
    #[arg(short = 'n', long = "newline", action=ArgAction::SetTrue)]
    newline: bool,
}

pub fn main(Options { text, newline }: &Options) -> Result<(), Error> {
    let t = text.join(" ");
    let sep = if newline == &true { "\n" } else { "" };
    if t.is_empty() {
        Err(Error::EmptyString)
    } else {
        print!("{t}{sep}");
        Ok(())
    }
}
