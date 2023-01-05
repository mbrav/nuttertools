use clap::{Parser, Subcommand};

use nuttertools::{phone_gen, prosecho, rat, Error};
use std::time::{Duration, Instant};

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
    PhoneGen(phone_gen::Options),
    Prosecho(prosecho::Options),
    Rat(rat::Options),
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let start: Instant = Instant::now();

    match &cli.command {
        Commands::PhoneGen(options) => phone_gen::main(options)?,
        Commands::Prosecho(options) => prosecho::main(options)?,
        Commands::Rat(options) => rat::main(options)?,
    }

    let end: Duration = start.elapsed();
    println!("Elapsed: {end:.3?}");

    Ok(())
}
