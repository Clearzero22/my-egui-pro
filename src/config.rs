use crate::theme::GruvboxTheme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: GruvboxTheme,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: GruvboxTheme::Dark,
        }
    }
}

pub struct Config {
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut config_path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        config_path.push("my_egui_pro");
        config_path.push("config.json");

        Self { config_path }
    }

    fn ensure_dir(&self) -> std::io::Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    pub fn load(&self) -> AppConfig {
        if !self.config_path.exists() {
            let default = AppConfig::default();
            if let Err(e) = self.save(&default) {
                eprintln!("Failed to create default config: {}", e);
            }
            return default;
        }

        fs::read_to_string(&self.config_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, config: &AppConfig) -> std::io::Result<()> {
        self.ensure_dir()?;

        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;

        Ok(())
    }
}
