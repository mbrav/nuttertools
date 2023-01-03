use clap::{Parser, Subcommand};

use nuttertools::{prosecho, rat, Error};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// A collection of crazy CLI tools in Rust
#[derive(Subcommand)]
pub enum Commands {
    Prosecho(prosecho::Options),
    Rat(rat::Options),
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Prosecho(prosecho_options) => prosecho::main(prosecho_options)?,
        Commands::Rat(rat_options) => rat::main(rat_options)?,
    }
    Ok(())
}
