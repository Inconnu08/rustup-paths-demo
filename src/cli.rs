use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "rustup-paths-demo")]
#[command(about = "Prototype path resolver for rustup XDG support...")]
pub struct Cli {
    #[arg(long)]
    pub use_xdg: bool,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Resolve,
    Explain,
    MigratePlan,
    Migrate {
        #[arg(long)]
        execute: bool,
    },
}