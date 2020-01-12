use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::path::PathBuf;
use std::fs;

use super::resource::Resource;

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize)]
pub struct Config {
    pub columns: u8,
    pub search_prompt: String,
    pub enable_pywal: bool,
}

impl Config {
    pub fn create_dir () {
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push("waffy");
            if !config_dir.exists() {
                let _ = fs::create_dir_all(config_dir);
            }
        }
    }

    pub fn default (path_to_save: Option<PathBuf>) -> Self {
//        let file = Resource::get("default_config.json5").unwrap();
//        let content = String::from_utf8(file.as_ref().to_vec())
        let content = Resource::from_file("default_config.json5")
            .expect("Cannot read default config");

        let config = json5::from_str::<Self>(&content).expect("Cannot parse default config");
        if let Some(path) = path_to_save {
            let _ = fs::write(path, &content);
        }

        config
    }

    pub fn get_uncached () -> Self {
        if let Some(mut config_path) = dirs::config_dir() {
            config_path.push("waffy");
            config_path.push("config");

            if !config_path.exists() {
                return Self::default(Some(config_path));
            }

            let content = fs::read_to_string(config_path).expect("Could not read config");
            let config = json5::from_str::<Self>(&content).expect("Could not parse config");
            return config;
        }

        Self::default(None)
    }


    pub fn get () -> &'static Self {
        CONFIG.get_or_init(Self::get_uncached)
    }
}

