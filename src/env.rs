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
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| "HOME is not set".to_string())?;

        Ok(Self {
            home,
            rustup_home: env::var_os("RUSTUP_HOME").map(PathBuf::from),
            cargo_home: env::var_os("CARGO_HOME").map(PathBuf::from),
            xdg_config_home: env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            xdg_data_home: env::var_os("XDG_DATA_HOME").map(PathBuf::from),
            xdg_cache_home: env::var_os("XDG_CACHE_HOME").map(PathBuf::from),
        })
    }

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

    pub fn default_rustup_home(&self) -> PathBuf {
        self.home.join(".rustup")
    }

    pub fn default_cargo_home(&self) -> PathBuf {
        self.home.join(".cargo")
    }

    pub fn effective_xdg_config_home(&self) -> PathBuf {
        self.xdg_config_home
            .clone()
            .unwrap_or_else(|| self.home.join(".config"))
    }

    pub fn effective_xdg_data_home(&self) -> PathBuf {
        self.xdg_data_home
            .clone()
            .unwrap_or_else(|| self.home.join(".local").join("share"))
    }

    pub fn effective_xdg_cache_home(&self) -> PathBuf {
        self.xdg_cache_home
            .clone()
            .unwrap_or_else(|| self.home.join(".cache"))
    }
}

pub fn join<P: AsRef<Path>>(base: P, child: &str) -> PathBuf {
    base.as_ref().join(child)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn from_values_uses_supplied_home_for_legacy_defaults() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);

        assert_eq!(env.default_rustup_home(), p("/home/alice/.rustup"));
        assert_eq!(env.default_cargo_home(), p("/home/alice/.cargo"));
    }

    #[test]
    fn from_values_uses_supplied_home_for_xdg_fallbacks() {
        let env = EnvPaths::from_values(p("/home/alice"), None, None, None, None, None);

        assert_eq!(env.effective_xdg_config_home(), p("/home/alice/.config"));
        assert_eq!(env.effective_xdg_data_home(), p("/home/alice/.local/share"));
        assert_eq!(env.effective_xdg_cache_home(), p("/home/alice/.cache"));
    }

    #[test]
    fn explicit_xdg_values_override_home_based_fallbacks() {
        let env = EnvPaths::from_values(
            p("/home/alice"),
            None,
            None,
            Some(p("/xdg/config")),
            Some(p("/xdg/data")),
            Some(p("/xdg/cache")),
        );

        assert_eq!(env.effective_xdg_config_home(), p("/xdg/config"));
        assert_eq!(env.effective_xdg_data_home(), p("/xdg/data"));
        assert_eq!(env.effective_xdg_cache_home(), p("/xdg/cache"));
    }
}