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

pub fn resolve_paths(
    env: &EnvPaths,
    config: &AppConfig,
    use_xdg: bool,
) -> Result<ResolutionReport, String> {
    // we should follow a precedence order:
    // - config file overrides takes the highest priority
    // - followed by env vars for backwards compatibility
    // - then optional XDG mode for modern layouts
    // - legacy defaults as fallback

    let mut decisions = Vec::new();
    let mut warnings = Vec::new();

    let overrides = config.overrides.clone().unwrap_or_default();

    // config dir
    let config_dir = if let Some(path) = overrides.config_dir {
        decisions.push(decision(
            "config_dir",
            path.clone(),
            "using config file override for config_dir",
        ));
        path
    } else if let Some(rustup_home) = &env.rustup_home {
        decisions.push(decision(
            "config_dir",
            rustup_home.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup config layout",
        ));
        rustup_home.clone()
    } else if use_xdg {
        let path = join(env.effective_xdg_config_home(), "rustup");
        decisions.push(decision(
            "config_dir",
            path.clone(),
            "XDG mode enabled and no RUSTUP_HOME override was set",
        ));
        path
    } else {
        let path = env.default_rustup_home();
        decisions.push(decision(
            "config_dir",
            path.clone(),
            "using legacy default rustup config layout",
        ));
        path
    };

    // data dir
    let data_dir = if let Some(path) = overrides.data_dir {
        decisions.push(decision(
            "data_dir",
            path.clone(),
            "using config file override for data_dir",
        ));
        path
    } else if let Some(rustup_home) = &env.rustup_home {
        decisions.push(decision(
            "data_dir",
            rustup_home.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup data layout",
        ));
        rustup_home.clone()
    } else if use_xdg {
        let path = join(env.effective_xdg_data_home(), "rustup");
        decisions.push(decision(
            "data_dir",
            path.clone(),
            "XDG mode enabled and no RUSTUP_HOME override was set",
        ));
        path
    } else {
        let path = env.default_rustup_home();
        decisions.push(decision(
            "data_dir",
            path.clone(),
            "using legacy default rustup data layout",
        ));
        path
    };

    // cache dir
    let cache_dir = if let Some(path) = overrides.cache_dir {
        decisions.push(decision(
            "cache_dir",
            path.clone(),
            "using config file override for cache_dir",
        ));
        path
    } else if let Some(rustup_home) = &env.rustup_home {
        let path = join(rustup_home, "tmp");
        decisions.push(decision(
            "cache_dir",
            path.clone(),
            "explicit RUSTUP_HOME override preserves legacy rustup cache/tmp layout",
        ));
        path
    } else if use_xdg {
        let path = join(env.effective_xdg_cache_home(), "rustup");
        decisions.push(decision(
            "cache_dir",
            path.clone(),
            "XDG mode enabled and no RUSTUP_HOME override was set",
        ));
        path
    } else {
        let path = join(env.default_rustup_home(), "tmp");
        decisions.push(decision(
            "cache_dir",
            path.clone(),
            "using legacy default rustup cache/tmp layout",
        ));
        path
    };

    // bin dir
    let bin_dir = if let Some(path) = overrides.bin_dir {
        decisions.push(decision(
            "bin_dir",
            path.clone(),
            "using config file override for bin_dir",
        ));
        path
    } else if let Some(cargo_home) = &env.cargo_home {
        let path = join(cargo_home, "bin");
        decisions.push(decision(
            "bin_dir",
            path.clone(),
            "explicit CARGO_HOME override controls cargo-compatible bin directory",
        ));
        path
    } else {
        let path = join(env.default_cargo_home(), "bin");
        decisions.push(decision(
            "bin_dir",
            path.clone(),
            "using legacy cargo bin path for compatibility",
        ));
        path
    };

    if env.rustup_home.is_some() && use_xdg {
        warnings.push(
            "RUSTUP_HOME is set; XDG mode does not change rustup directories unless config overrides are used."
                .to_string(),
        );
    }

    if env.cargo_home.is_some() {
        warnings.push(
            "CARGO_HOME is set; bin_dirr follows legacy cargo behavior unless explicitly overridden in config."
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
    fn xdg_mode_without_overrides() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);
        let cfg = AppConfig::default();

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.config_dir, p("/home/alice/.config/rustup"));
        assert_eq!(report.resolved.data_dir, p("/home/alice/.local/share/rustup"));
        assert_eq!(report.resolved.cache_dir, p("/home/alice/.cache/rustup"));
        assert_eq!(report.resolved.bin_dir, p("/home/alice/.cargo/bin"));
    }

    #[test]
    fn explicit_rustup_home_wins() {
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
    }

    #[test]
    fn config_override_wins() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);
        let cfg = AppConfig {
            overrides: Some(PathOverrides {
                config_dir: Some(p("/override/config")),
                data_dir: None,
                cache_dir: None,
                bin_dir: None,
            }),
        };

        let report = resolve_paths(&env, &cfg, true).unwrap();
        assert_eq!(report.resolved.config_dir, p("/override/config"));
    }
}
