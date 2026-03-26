mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Resolve => {
            println!("resolve command");
        }
        Commands::Explain => {
            println!("explain command");
        }
        Commands::MigratePlan => {
            println!("migrate-plan command");
        }
        Commands::Migrate { execute } => {
            println!("migrate command, execute={execute}");
        }
    }
}