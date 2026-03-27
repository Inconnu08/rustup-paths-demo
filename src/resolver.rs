use crate::config::AppConfig;
use crate::env::{EnvPaths, join};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub bin_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct PathDecision {
    pub name: String,
    pub selected_path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolutionReport {
    pub resolved: ResolvedPaths,
    pub decisions: Vec<PathDecision>,
    pub warnings: Vec<String>,
}

/// a compatibility-first precedence model
///
/// Precedence order:
/// 1. Explicit env overrides:
///    - RUSTUP_HOME (controls config/data/cache layout)
///    - CARGO_HOME (controls bin/shim directory)
///
/// 2. Legacy defaults:
///    - ~/.rustup
///    - ~/.cargo/bin
///
/// 3. XDG paths (only when `use_xdg` is explicitly enabled):
///    - ~/.config/rustup
///    - ~/.local/share/rustup
///    - ~/.cache/rustup
///
/// - in this way we can preserve rustup’s current behavior by default (no breaking changes).
/// - XDG support is introduced as an opt-in mode, not a default.
/// - Config-file overrides are treated as prototype-only and are not part of rustup's real precedence...just experimenting.
/// - Bin/shim handling remains aligned with CARGO_HOME for compatibility with Cargo, keeping it conservative.
/// - This structure is intended to support future refactoring toward XDG compliance without altering existing installs.
pub fn resolve_paths(
    env: &EnvPaths,
    config: &AppConfig,
    use_xdg: bool,
) -> Result<ResolutionReport, String> {
    let mut decisions = Vec::new();
    let mut warnings = Vec::new();

    let overrides = config.overrides.clone().unwrap_or_default();

    // prototype note:
    // these config overrides are not meant to model rustup's real external precedence.
    // They are only a local escape hatch for experimentation.
    let has_any_config_override = overrides.config_dir.is_some()
        || overrides.data_dir.is_some()
        || overrides.cache_dir.is_some()
        || overrides.bin_dir.is_some();

    if has_any_config_override {
        println!("{}", "config-file path overrides are simply a prototype-only and should not be treated as rustup's user-facing precedence".to_string());
        warnings.push(
            "config-file path overrides are simply a prototype-only and should not be treated as rustup's user-facing precedence"
                .to_string(),
        );
    }

    // config_dir
    let config_dir = if let Some(rustup_home) = &env.rustup_home {
        decisions.push(decision(
            "config_dir",
            rustup_home.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup layout",
        ));
        rustup_home.clone()
    } else if use_xdg {
        let has_config_override = overrides.config_dir.is_some();
        let path = overrides
            .config_dir
            .clone()
            .unwrap_or_else(|| join(env.effective_xdg_config_home(), "rustup"));

        decisions.push(decision(
            "config_dir",
            path.clone(),
            if has_config_override {
                "XDG mode enabled; using prototype config_dir override"
            } else {
                "XDG mode explicitly enabled and no RUSTUP_HOME override was set"
            },
        ));
        path
    } else {
        let has_config_override = overrides.config_dir.is_some();
        let path = overrides
            .config_dir
            .clone()
            .unwrap_or_else(|| env.default_rustup_home());

        decisions.push(decision(
            "config_dir",
            path.clone(),
            if has_config_override {
                "legacy mode; using prototype config_dir override"
            } else {
                "preserving legacy default rustup config layout"
            },
        ));
        path
    };

    // data_dir
    let data_dir = if let Some(rustup_home) = &env.rustup_home {
        decisions.push(decision(
            "data_dir",
            rustup_home.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup layout",
        ));
        rustup_home.clone()
    } else if use_xdg {
        let has_data_override = overrides.data_dir.is_some();
        let path = overrides
            .data_dir
            .clone()
            .unwrap_or_else(|| join(env.effective_xdg_data_home(), "rustup"));

        decisions.push(decision(
            "data_dir",
            path.clone(),
            if has_data_override {
                "XDG mode enabled; using prototype data_dir override"
            } else {
                "XDG mode explicitly enabled and no RUSTUP_HOME override was set"
            },
        ));
        path
    } else {
        let has_data_override = overrides.data_dir.is_some();
        let path = overrides
            .data_dir
            .clone()
            .unwrap_or_else(|| env.default_rustup_home());

        decisions.push(decision(
            "data_dir",
            path.clone(),
            if has_data_override {
                "legacy mode; using prototype data_dir override"
            } else {
                "preserving legacy default rustup data layout"
            },
        ));
        path
    };

    // cache_dir
    let cache_dir = if let Some(rustup_home) = &env.rustup_home {
        let path = join(rustup_home, "tmp");
        decisions.push(decision(
            "cache_dir",
            path.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup cache/tmp layout",
        ));
        path
    } else if use_xdg {
        let has_cache_override = overrides.cache_dir.is_some();
        let path = overrides
            .cache_dir
            .clone()
            .unwrap_or_else(|| join(env.effective_xdg_cache_home(), "rustup"));

        decisions.push(decision(
            "cache_dir",
            path.clone(),
            if has_cache_override {
                "XDG mode enabled; using prototype cache_dir override"
            } else {
                "XDG mode explicitly enabled and no RUSTUP_HOME override was set"
            },
        ));
        path
    } else {
        let has_cache_override = overrides.cache_dir.is_some();
        let path = overrides
            .cache_dir
            .clone()
            .unwrap_or_else(|| join(env.default_rustup_home(), "tmp"));

        decisions.push(decision(
            "cache_dir",
            path.clone(),
            if has_cache_override {
                "legacy mode; using prototype cache_dir override"
            } else {
                "preserving legacy default rustup cache/tmp layout"
            },
        ));
        path
    };

    // bin_dir
    let bin_dir = if let Some(cargo_home) = &env.cargo_home {
        let path = join(cargo_home, "bin");
        decisions.push(decision(
            "bin_dir",
            path.clone(),
            "explicit CARGO_HOME override preserves cargo-compatible bin directory",
        ));
        path
    } else {
        let has_bin_override = overrides.bin_dir.is_some();
        let path = overrides
            .bin_dir
            .clone()
            .unwrap_or_else(|| join(env.default_cargo_home(), "bin"));

        decisions.push(decision(
            "bin_dir",
            path.clone(),
            if has_bin_override {
                "using prototype bin_dir override; legacy rustup default remains ~/.cargo/bin"
            } else {
                "preserving legacy cargo-compatible bin path for compatibility"
            },
        ));
        path
    };

    if env.rustup_home.is_some() && use_xdg {
        println!("{}", env.rustup_home.clone().unwrap().display());
        warnings.push(
            "RUSTUP_HOME is set, so rustup-owned directories stay in legacy layout even when XDG mode is enabled"
                .to_string(),
        );
    }

    if env.cargo_home.is_some() {
        warnings.push("CARGO_HOME is set, so bin_dir follows legacy cargo behavior".to_string());
    }

    if use_xdg && overrides.bin_dir.is_none() {
        warnings.push(
            "XDG mode currently does not move bin_dir; keeping ~/.cargo/bin-style behavior is the conservative default"
                .to_string(),
        );
    }

    Ok(ResolutionReport {
        resolved: ResolvedPaths {
            config_dir,
            data_dir,
            cache_dir,
            bin_dir,
        },
        decisions,
        warnings,
    })
}

fn decision(name: &str, selected_path: PathBuf, reason: &str) -> PathDecision {
    PathDecision {
        name: name.to_string(),
        selected_path,
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, PathOverrides};
    use std::path::PathBuf;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn legacy_mode_without_overrides_preserves_current_defaults() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);
        let cfg = AppConfig::default();

        let report = resolve_paths(&env, &cfg, false).unwrap();
        assert_eq!(report.resolved.config_dir, p("/home/alice/.rustup"));
        assert_eq!(report.resolved.data_dir, p("/home/alice/.rustup"));
        assert_eq!(report.resolved.cache_dir, p("/home/alice/.rustup/tmp"));
        assert_eq!(report.resolved.bin_dir, p("/home/alice/.cargo/bin"));
    }

    #[test]
    fn xdg_mode_without_env_overrides_uses_xdg_split() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);
        let cfg = AppConfig::default();

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.config_dir, p("/home/alice/.config/rustup"));
        assert_eq!(
            report.resolved.data_dir,
            p("/home/alice/.local/share/rustup")
        );
        assert_eq!(report.resolved.cache_dir, p("/home/alice/.cache/rustup"));
        assert_eq!(report.resolved.bin_dir, p("/home/alice/.cargo/bin"));
    }

    #[test]
    fn explicit_rustup_home_wins_even_when_xdg_mode_is_enabled() {
        let env = EnvPaths::from_values(
            p("/home/alice"),
            Some(p("/custom/rustup")),
            None,
            None,
            None,
            None,
        );
        let cfg = AppConfig::default();

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.config_dir, p("/custom/rustup"));
        assert_eq!(report.resolved.data_dir, p("/custom/rustup"));
        assert_eq!(report.resolved.cache_dir, p("/custom/rustup/tmp"));
        assert_eq!(report.resolved.bin_dir, p("/home/alice/.cargo/bin"));
    }

    #[test]
    fn explicit_cargo_home_wins_for_bin_dir() {
        let env = EnvPaths::from_values(
            p("/home/alice"),
            None,
            Some(p("/custom/cargo")),
            None,
            None,
            None,
        );
        let cfg = AppConfig::default();

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.bin_dir, p("/custom/cargo/bin"));
    }

    #[test]
    fn prototype_config_override_does_not_beat_env() {
        let env = EnvPaths::from_values(
            p("/home/alice"),
            Some(p("/custom/rustup")),
            None,
            None,
            None,
            None,
        );

        let cfg = AppConfig {
            overrides: Some(PathOverrides {
                config_dir: Some(p("/override/config")),
                data_dir: Some(p("/override/data")),
                cache_dir: Some(p("/override/cache")),
                bin_dir: Some(p("/override/bin")),
            }),
        };

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.config_dir, p("/custom/rustup"));
        assert_eq!(report.resolved.data_dir, p("/custom/rustup"));
        assert_eq!(report.resolved.cache_dir, p("/custom/rustup/tmp"));
        assert_eq!(report.resolved.bin_dir, p("/override/bin"));
    }
}
