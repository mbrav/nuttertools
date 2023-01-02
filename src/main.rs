use clap::{Parser, Subcommand};

use nuttertools::echo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// nuttertools
    /// A collection of crazy but professional CLI tools in Rust
    ///
    /// Echo program
    Echo(echo::Options),
    /// Telephone number generator
    TelNumGen,
}

fn main() -> Result<(), echo::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo(echo_options) => echo::main(echo_options)?,
        Commands::TelNumGen => println!("Cool"),
    }
    Ok(())
}
