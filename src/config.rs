use confy::{load_path, store_path};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};
use crate::printer;

#[derive(Debug)]
pub enum ConfigError {
    UserHome,
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::UserHome => write!(f, "HOME path is invalid"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub repo_path: String,
    pub slack_web_hook: String,
}

impl Config {
    pub fn new(repo_path: &str, slack_web_hook: &str) -> Self {
        Self {
            repo_path: repo_path.into(),
            slack_web_hook: slack_web_hook.into(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "{{\n  repo_path: {}\n  slack_web_hook: {}\n}}",
            self.repo_path, self.slack_web_hook
        );
        f.write_str(&string)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: "".into(),
            slack_web_hook: "".into(),
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let dirs = UserDirs::new().ok_or(ConfigError::UserHome)?;
    let path = dirs.home_dir();
    let mut path_buf = path.to_path_buf();
    path_buf.push(".config");
    path_buf.push(super::APP_NAME);
    path_buf.push("config");
    path_buf.set_extension("toml");
    load_path(&path_buf.as_path()).map_err(|e| e.into())
}

pub fn store_config(config: &Config) -> Result<(), Box<dyn Error>> {
    printer::print_config(&config);
    let dirs = UserDirs::new().ok_or(ConfigError::UserHome)?;
    let path = dirs.home_dir();
    let mut path_buf = path.to_path_buf();
    path_buf.push(".config");
    path_buf.push(super::APP_NAME);
    path_buf.push("config");
    path_buf.set_extension("toml");
    store_path(&path_buf.as_path(), config).map_err(|e| e.into())
}
