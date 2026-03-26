mod cli;
mod env;

use clap::Parser;
use cli::{Cli, Commands};
use env::EnvPaths;

fn main() {
    let cli = Cli::parse();

    let env = match EnvPaths::from_system() {
        Ok(env) => env,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Resolve => {
            println!("{env:#?}");
        }
        Commands::Explain => {
            println!("{env:#?}");
        }
        Commands::MigratePlan => {
            println!("{env:#?}");
        }
        Commands::Migrate { execute } => {
            println!("migrate command, execute={execute}");
            println!("{env:#?}");
        }
    }
}