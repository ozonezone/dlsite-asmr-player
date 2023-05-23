use std::path::PathBuf;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs::create_dir_all;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub password: String,
    pub scan_dir: Vec<PathBuf>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            password: "password".to_string(),
            scan_dir: vec![],
        }
    }
}

pub(crate) static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("dlsite-asmr-player");
    config_dir
});

impl Config {
    pub(crate) async fn from_file() -> Result<Self> {
        let config_file_path = CONFIG_DIR.join("config.toml");
        let config_file = tokio::fs::read_to_string(config_file_path).await?;
        let config: Self = toml::from_str(&config_file)?;

        Ok(config)
    }
    pub(crate) async fn write_to_file(&self) -> Result<PathBuf> {
        let config_file_path = CONFIG_DIR.join("config.toml");

        if !tokio::fs::try_exists(&*CONFIG_DIR).await? {
            create_dir_all(&*CONFIG_DIR).await?;
        }

        let config_str = toml::to_string(&self)?;
        tokio::fs::write(&config_file_path, config_str).await?;

        Ok(config_file_path)
    }
}
