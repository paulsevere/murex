use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_template: String,
    pub projects_dir: PathBuf,
    pub auto_build: bool,
    pub editor: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        
        Self {
            default_template: "rust".to_string(),
            projects_dir: home_dir.join("murex-projects"),
            auto_build: false,
            editor: std::env::var("EDITOR").ok(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_file_path()?;
        
        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir()?;
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        // Ensure projects directory exists
        if !self.projects_dir.exists() {
            fs::create_dir_all(&self.projects_dir)?;
        }
        
        let config_path = get_config_file_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        
        Ok(())
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("murex");
    
    Ok(config_dir)
}

pub fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.toml"))
}

pub fn get_projects_file_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("projects.json"))
}
