use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Your Telegram user ID (the bot will only accept messages from this user)
    pub allowed_user_id: u64,

    /// The channel ID where videos will be posted (e.g., -1001234567890)
    pub channel_id: i64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "config.json";

        let config_str = fs::read_to_string(config_path)
            .context(format!("Failed to read config file: {}", config_path))?;

        let config: Config = serde_json::from_str(&config_str)
            .context("Failed to parse config file")?;

        Ok(config)
    }
}
