mod cli;
mod env;
mod migrate;
mod report;
mod resolver;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::env::EnvPaths;
use crate::migrate::build_migration_plan;
use crate::report::{print_explain, print_migration_plan, print_resolve};
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
        Commands::Resolve => print_resolve(&report, cli.json),
        Commands::Explain => print_explain(&report, cli.json),
        Commands::MigratePlan => {
            let plan = build_migration_plan(&report, &env);
            print_migration_plan(&plan, cli.json);
        }
        Commands::Migrate { execute } => {
            let plan = build_migration_plan(&report, &env);
            println!("migrate command, execute={execute}");
            print_migration_plan(&plan, cli.json);
        }
    }
}