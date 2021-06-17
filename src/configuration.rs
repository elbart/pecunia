use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub iex_api_token: String,
    pub database: Database,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();

        c.merge(
            File::with_name(&format!(
                "{}/.config/pecunia/config.toml",
                home_dir().unwrap_or_default().to_str().unwrap_or("")
            ))
            .required(false),
        )?;
        c.merge(File::with_name("config.toml").required(false))?;
        c.merge(Environment::with_prefix("PECUNIA"))?;

        c.try_into()
    }
}
