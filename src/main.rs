mod cli;
mod env;
mod resolver;

use clap::Parser;
use crate::cli::{Cli, Commands};
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

    let report = match resolve_paths(&env, cli.use_xdg) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Resolve => println!("{:#?}", report.resolved),
        Commands::Explain => println!("{:#?}", report),
        Commands::MigratePlan => println!("{:#?}", report),
        Commands::Migrate { execute } => {
            println!("migrate command, execute={execute}");
            println!("{:#?}", report);
        }
    }
}