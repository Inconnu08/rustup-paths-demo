use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct EnvPaths {
    pub home: PathBuf,
    pub rustup_home: Option<PathBuf>,
    pub cargo_home: Option<PathBuf>,
    pub xdg_config_home: Option<PathBuf>,
    pub xdg_data_home: Option<PathBuf>,
    pub xdg_cache_home: Option<PathBuf>,
}

impl EnvPaths {
    pub fn from_system() -> Result<Self, String> {
        let home = dirs::home_dir().ok_or_else(|| "could not determine home directory".to_string())?;

        Ok(Self {
            home,
            rustup_home: env::var_os("RUSTUP_HOME").map(PathBuf::from),
            cargo_home: env::var_os("CARGO_HOME").map(PathBuf::from),
            xdg_config_home: env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            xdg_data_home: env::var_os("XDG_DATA_HOME").map(PathBuf::from),
            xdg_cache_home: env::var_os("XDG_CACHE_HOME").map(PathBuf::from),
        })
    }

    pub fn default_rustup_home(&self) -> PathBuf {
        self.home.join(".rustup")
    }

    pub fn default_cargo_home(&self) -> PathBuf {
        self.home.join(".cargo")
    }

    pub fn effective_xdg_config_home(&self) -> PathBuf {
        self.xdg_config_home
            .clone()
            .or_else(dirs::config_dir)
            .unwrap_or_else(|| self.home.join(".config"))
    }

    pub fn effective_xdg_data_home(&self) -> PathBuf {
        self.xdg_data_home
            .clone()
            .or_else(dirs::data_dir)
            .unwrap_or_else(|| self.home.join(".local").join("share"))
    }

    pub fn effective_xdg_cache_home(&self) -> PathBuf {
        self.xdg_cache_home
            .clone()
            .or_else(dirs::cache_dir)
            .unwrap_or_else(|| self.home.join(".cache"))
    }

    #[cfg(test)]
    pub fn from_values(
        home: impl Into<PathBuf>,
        rustup_home: Option<PathBuf>,
        cargo_home: Option<PathBuf>,
        xdg_config_home: Option<PathBuf>,
        xdg_data_home: Option<PathBuf>,
        xdg_cache_home: Option<PathBuf>,
    ) -> Self {
        Self {
            home: home.into(),
            rustup_home,
            cargo_home,
            xdg_config_home,
            xdg_data_home,
            xdg_cache_home,
        }
    }
}

pub fn join<P: AsRef<Path>>(base: P, child: &str) -> PathBuf {
    base.as_ref().join(child)
}