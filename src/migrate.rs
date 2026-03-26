use serde::Serialize;

use crate::env::EnvPaths;
use crate::resolver::ResolutionReport;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct MigrationStep {
    pub from: PathBuf,
    pub to: PathBuf,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationPlan {
    pub warnings: Vec<String>,
    pub steps: Vec<MigrationStep>,
}

pub fn build_migration_plan(report: &ResolutionReport, env: &EnvPaths) -> MigrationPlan {
    let mut warnings = report.warnings.clone();
    let mut steps = Vec::new();

    if env.rustup_home.is_some() {
        warnings.push(
            "RUSTUP_HOME is explicitly set; migration may be undesirable.".to_string(),
        );
    }

    let legacy_rustup_home = env.default_rustup_home();

    add_step(
        &mut steps,
        legacy_rustup_home.join("toolchains"),
        report.resolved.data_dir.join("toolchains"),
        "Move installed toolchains to the data directory",
    );

    add_step(
        &mut steps,
        legacy_rustup_home.join("settings.toml"),
        report.resolved.config_dir.join("settings.toml"),
        "Move settings to the config directory",
    );

    add_step(
        &mut steps,
        legacy_rustup_home.join("downloads"),
        report.resolved.cache_dir.join("downloads"),
        "Move downloads to the cache directory",
    );

    MigrationPlan { warnings, steps }
}

fn add_step(steps: &mut Vec<MigrationStep>, from: PathBuf, to: PathBuf, description: &str) {
    if from == to {
        return;
    }

    steps.push(MigrationStep {
        from,
        to,
        description: description.to_string(),
    });
}