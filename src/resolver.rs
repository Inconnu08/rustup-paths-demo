use crate::env::{join, EnvPaths};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ResolvedPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub bin_dir: PathBuf,
}

pub fn resolve_paths(env: &EnvPaths, use_xdg: bool) -> Result<ResolvedPaths, String> {
    // we should follow a precedence order:
    // - config file overrides takes the highest priority
    // - followed by env vars for backwards compatibility
    // - then optional XDG mode for modern layouts
    // - legacy defaults as fallback

    // config dir
    let config_dir = if let Some(rustup_home) = &env.rustup_home {
        rustup_home.clone()
    } else if use_xdg {
        join(env.effective_xdg_config_home(), "rustup")
    } else {
        env.default_rustup_home()
    };

    // data dir
    let data_dir = if let Some(rustup_home) = &env.rustup_home {
        rustup_home.clone()
    } else if use_xdg {
        join(env.effective_xdg_data_home(), "rustup")
    } else {
        env.default_rustup_home()
    };

    // cache dir
    let cache_dir = if let Some(rustup_home) = &env.rustup_home {
        join(rustup_home, "tmp")
    } else if use_xdg {
        join(env.effective_xdg_cache_home(), "rustup")
    } else {
        join(env.default_rustup_home(), "tmp")
    };

    // bin dir
    let bin_dir = if let Some(cargo_home) = &env.cargo_home {
        join(cargo_home, "bin")
    } else {
        join(env.default_cargo_home(), "bin")
    };

    Ok(ResolvedPaths {
        config_dir,
        data_dir,
        cache_dir,
        bin_dir,
    })
}