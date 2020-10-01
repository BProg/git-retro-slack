use crate::{fs::get_config_file, DynErrResult};
use confy::{load_path, store_path};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

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

    pub fn load() -> DynErrResult<Config> {
        let file_path = Config::get_file_path()?;
        load_path(file_path.as_path()).map_err(Box::from)
    }

    pub fn store(&self) -> DynErrResult<()> {
        let file_path = Config::get_file_path()?;
        store_path(file_path.as_path(), self).map_err(Box::from)
    }

    fn get_file_path() -> DynErrResult<PathBuf> {
        #[cfg(feature = "production")]
        let file = "config";
        #[cfg(not(feature = "production"))]
        let file = "config_dev";
        get_config_file(file)
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
