use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub languages: Vec<Language>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Language {
    pub language: String,
    pub extention: String,
    pub template: Option<String>,
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("Failed to get config directory.")
        .join("kyopro_folder")
        .with_extension("toml")
}

pub fn make_new_config(path: PathBuf) {
    let config = include_str!("../config/kyopro_folder.toml");
    fs::write(path, config).expect("Failed to write to new config file.");
}

pub fn read_config(path: PathBuf) -> Config {
    let toml = fs::read_to_string(path).expect("Failed to read toml file.");
    let config = toml::from_str::<Config>(&toml).expect("Failed to parse toml file.");
    config
}
