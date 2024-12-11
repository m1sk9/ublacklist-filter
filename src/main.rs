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
    /// Test if the rules can be parsed correctly. You will be warned if there are duplicate domains.
    Test {
        /// Displays the domains evaluated at the time of testing. Depending on the number of rules, the terminal will flow.
        #[clap(short, long, default_value = "false")]
        verbose: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build => {
            if let Err(why) = command::build::execute() {
                eprintln!("Error: {}", why);
            }
        }
        Commands::Test { verbose } => {
            if let Err(why) = command::test::execute(*verbose) {
                eprintln!("Error: {}", why);
            }
        }
    }
}
