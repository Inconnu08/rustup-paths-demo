mod cli;
mod env;
mod migrate;
mod resolver;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::env::EnvPaths;
use crate::migrate::build_migration_plan;
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
        Commands::MigratePlan => {
            let plan = build_migration_plan(&report, &env);
            println!("{:#?}", plan);
        }
        Commands::Migrate { execute } => {
            let plan = build_migration_plan(&report, &env);
            println!("migrate command, execute={execute}");
            println!("{:#?}", plan);
        }
    }
}