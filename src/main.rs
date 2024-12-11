mod command;
mod model;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a filter from a rule definition file. If there are duplicate domains, they will not be created.
    Build,
    /// Test if the rules can be parsed correctly.
    Test,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build => {
            if let Err(why) = command::build::execute() {
                eprintln!("Error: {}", why);
            }
        }
        Commands::Test => {
            if let Err(why) = command::test::execute() {
                eprintln!("Error: {}", why);
            }
        }
    }
}
