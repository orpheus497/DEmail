use crate::error::DEmailError;
use crate::models::OAuthProviderConfig;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::Config;

fn get_config_path(config: &Config) -> Result<PathBuf, DEmailError> {
    let path = tauri::api::path::app_config_dir(config)
        .ok_or_else(|| DEmailError::Init("Could not determine app config directory".to_string()))?;
    Ok(path.join("demail_config.json"))
}

pub fn load_config(config: &Config) -> Result<HashMap<String, OAuthProviderConfig>, DEmailError> {
    let path = get_config_path(config)?;
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let content = fs::read_to_string(path)?;
    let configs = serde_json::from_str(&content)?;
    Ok(configs)
}

pub fn save_config(
    config: &Config,
    configs: &HashMap<String, OAuthProviderConfig>,
) -> Result<(), DEmailError> {
    let path = get_config_path(config)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(configs)?;
    fs::write(path, content)?;
    Ok(())
}
