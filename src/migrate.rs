use crate::env::EnvPaths;
use crate::resolver::ResolutionReport;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct MigrationStep {
    pub from: PathBuf,
    pub to: PathBuf,
    pub description: String,
    pub source_exists: bool,
    pub destination_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationPlan {
    pub warnings: Vec<String>,
    pub steps: Vec<MigrationStep>,
}

#[derive(Debug)]
pub struct MigrationSummary {
    pub moved: usize,
    pub skipped: usize,
    pub messages: Vec<String>,
}

pub fn build_migration_plan(report: &ResolutionReport, env: &EnvPaths) -> MigrationPlan {
    let mut warnings = report.warnings.clone();
    let mut steps = Vec::new();

    if env.rustup_home.is_some() {
        warnings.push(
            "RUSTUP_HOME is explicitly set; migration is usually not recommended.".to_string(),
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

pub fn apply_migration_plan(plan: &MigrationPlan) -> Result<MigrationSummary, String> {
    let mut moved = 0usize;
    let mut skipped = 0usize;
    let mut messages = Vec::new();

    for step in &plan.steps {
        if !step.source_exists {
            skipped += 1;
            messages.push(format!(
                "skipped {} because source does not exist",
                step.from.display()
            ));
            continue;
        }

        if step.destination_exists {
            skipped += 1;
            messages.push(format!(
                "skipped {} because destination already exists: {}",
                step.from.display(),
                step.to.display()
            ));
            continue;
        }

        if let Some(parent) = step.to.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
        }

        fs::rename(&step.from, &step.to).map_err(|e| {
            format!(
                "failed to move {} -> {}: {e}",
                step.from.display(),
                step.to.display()
            )
        })?;

        moved += 1;
        messages.push(format!(
            "moved {} -> {}",
            step.from.display(),
            step.to.display()
        ));
    }

    Ok(MigrationSummary {
        moved,
        skipped,
        messages,
    })
}

fn add_step(steps: &mut Vec<MigrationStep>, from: PathBuf, to: PathBuf, description: &str) {
    if from == to {
        return;
    }

    steps.push(MigrationStep {
        source_exists: from.exists(),
        destination_exists: to.exists(),
        from,
        to,
        description: description.to_string(),
    });
}