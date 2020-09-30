use std::{error, path::PathBuf};

use config::ConfigError;
use confy::{load_path, store_path};
use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::{config, git::RepoAnalyzer, send_to_slack, slack};

use super::log;

#[derive(Serialize, Deserialize, Default)]
pub struct RunDaemon {
    did_run: bool,
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let RunDaemon{did_run} = read_did_run()?;
    if did_run {
        return write_did_run(RunDaemon{did_run: false});
    }
    let app_config = config::get_config()?;
    log::multiple(vec![
        log::Style::Message("Config: "),
        log::Style::Important(&app_config.to_string()),
    ]);
    let repo = RepoAnalyzer::new(&app_config.repo_path);
    let commits = repo.get_commits()?;
    let branches = repo.get_in_progress()?;
    let message = slack::message::create_message(commits, branches);
    send_to_slack(app_config.slack_web_hook, message).map(|_| ())?;
    return write_did_run(RunDaemon{did_run: true});
}

fn read_did_run() -> Result<RunDaemon, Box<dyn error::Error>> {
    let file = get_did_run_file()?;
    load_path(file.as_path()).map_err(|e| e.into())
}

fn write_did_run(ran: RunDaemon) -> Result<(), Box<dyn error::Error>> {
    let file = get_did_run_file()?;
    store_path(file.as_path(), ran).map_err(|e| e.into())
}

fn get_did_run_file() -> Result<PathBuf, Box<dyn error::Error>> {
    let dirs = UserDirs::new().ok_or(ConfigError::UserHome)?;
    let path = dirs.home_dir();
    let mut path_buf = path.to_path_buf();
    path_buf.push(".config");
    path_buf.push(crate::APP_NAME);
    #[cfg(production)]
    path_buf.push("did_run");
    #[cfg(not(production))]
    path_buf.push("did_run_dev");
    path_buf.set_extension("toml");
    Ok(path_buf)
}
