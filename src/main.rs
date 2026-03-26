mod cli;
mod env;
mod resolver;

use clap::Parser;
use cli::{Cli, Commands};
use crate::env::EnvPaths;
use crate::resolver::resolve_paths;

fn main() {
    let cli = Cli::parse();

    let env = match EnvPaths::from_system() {
        Ok(env) => env,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    let resolved = match resolve_paths(&env, cli.use_xdg) {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Resolve => {
            println!("{resolved:#?}");
        }
        Commands::Explain => {
            println!("{resolved:#?}");
        }
        Commands::MigratePlan => {
            println!("{resolved:#?}");
        }
        Commands::Migrate { execute } => {
            println!("migrate command, execute={execute}");
            println!("{resolved:#?}");
        }
    }
}