use clap::{ArgAction, Parser, Subcommand};

use nuttertools::{phone_gen, prosecho, rat, russian_roulette, Error};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Specify whether to print elapsed time of the program
    #[arg(short, long, action=ArgAction::SetTrue)]
    no_time: bool,
}

/// A collection of crazy CLI tools in Rust
#[derive(Subcommand)]
pub enum Commands {
    PhoneGen(phone_gen::Options),
    Prosecho(prosecho::Options),
    Rat(rat::Options),
    RussianRoulette(russian_roulette::Options),
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let start: Instant = Instant::now();

    match &cli.command {
        Commands::PhoneGen(options) => phone_gen::main(options)?,
        Commands::Prosecho(options) => prosecho::main(options)?,
        Commands::Rat(options) => rat::main(options)?,
        Commands::RussianRoulette(options) => russian_roulette::main(options)?,
    }

    if !&cli.no_time {
        let end: Duration = start.elapsed();
        println!("Elapsed: {end:.3?}");
    }

    Ok(())
}
