use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub megahealth_warning_threshold: u32,
    pub megahealth_critical_threshold: u32,
    pub red_armor_warning_threshold: u32,
    pub red_armor_critical_threshold: u32,
    pub megahealth_hotkey: String,
    pub red_armor_hotkey: String,
}

pub fn load_config() -> Result<Config, String> {
    let content = match fs::read_to_string(get_config_path()?) {
        Ok(content) => content,
        Err(_) => return write_default_config(),
    };

    let config = toml::from_str::<Config>(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    Ok(config)
}

fn get_config_path() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|home| home.join("quake-tools.toml"))
        .ok_or_else(|| "Could not determine home directory".to_string())
}

fn write_default_config() -> Result<Config, String> {
    let default_config = Config {
        megahealth_warning_threshold: 10,
        megahealth_critical_threshold: 5,
        red_armor_warning_threshold: 10,
        red_armor_critical_threshold: 5,
        megahealth_hotkey: "Key1".to_string(),
        red_armor_hotkey: "Key2".to_string(),
    };

    let toml_content = toml::to_string(&default_config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(get_config_path()?, toml_content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(default_config)
}
