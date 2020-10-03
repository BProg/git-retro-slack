use super::log;
use crate::{
    config, fs::get_savedata_file, git, git::RepoAnalyzer, slack, DynErrResult,
};
use config::Config;
use confy::{load_path, store_path};
use git::search_interval::SearchInterval;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct RunDaemon {
    did_run: bool,
}

pub fn run() -> DynErrResult<()> {
    let RunDaemon { did_run } = read_did_run()?;
    if did_run {
        return write_did_run(RunDaemon { did_run: false });
    }
    let app_config = Config::load()?;
    log::multiple(vec![
        log::Style::Message("Config: "),
        log::Style::Important(&app_config.to_string()),
    ]);
    let repo = RepoAnalyzer::new(&app_config.repo_path)?;
    let SearchInterval { from, to } = repo.interval;
    log::multiple(vec![
        log::Style::Message("Searching logs from: "),
        log::Style::Important(&from.to_string()),
        log::Style::Message(" to "),
        log::Style::Important(&to.to_string()),
    ]);
    let commits = repo.get_commits()?;
    let branches = repo.get_in_progress()?;
    let message = slack::Message {
        branches,
        commits,
        interval: repo.interval
    };
    let response = message.send_to_slack(app_config.slack_web_hook, message.format_slack())?;
    log::message(format!("{:?}", &response));
    write_did_run(RunDaemon { did_run: true })
}

fn read_did_run() -> DynErrResult<RunDaemon> {
    let file = get_savedata_file()?;
    load_path(file.as_path()).map_err(|e| e.into())
}

fn write_did_run(ran: RunDaemon) -> DynErrResult<()> {
    let file = get_savedata_file()?;
    store_path(file.as_path(), ran).map_err(|e| e.into())
}
