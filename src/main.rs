mod cli;
mod env;
mod migrate;
mod report;
mod resolver;

use clap::Parser;
use cli::{Cli, Commands};
use env::EnvPaths;
use migrate::{apply_migration_plan, build_migration_plan};
use report::{print_explain, print_migration_plan, print_resolve};
use resolver::resolve_paths;

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

            if !execute {
                println!("Dry run only. Use `migrate --execute` to perform filesystem changes.\n");
                print_migration_plan(&plan, cli.json);
                return;
            }

            match apply_migration_plan(&plan) {
                Ok(summary) => {
                    println!("Migration completed.");
                    println!("moved:   {}", summary.moved);
                    println!("skipped: {}", summary.skipped);
                    println!();
                    for msg in summary.messages {
                        println!("- {msg}");
                    }
                }
                Err(err) => {
                    eprintln!("migration failed: {err}");
                    std::process::exit(1);
                }
            }
        }
    }
}