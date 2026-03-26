use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub overrides: Option<PathOverrides>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathOverrides {
    pub config_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub cache_dir: Option<PathBuf>,
    pub bin_dir: Option<PathBuf>,
}

impl AppConfig {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        toml::from_str(&raw)
            .map_err(|e| format!("failed to parse TOML {}: {e}", path.display()))
    }
}
