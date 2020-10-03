mod cli;
mod config;
mod fs;
mod git;
mod launchd;
mod slack;

use cli::{configure, log, Command};
use config::Config;
use git::{search_interval::SearchInterval, RepoAnalyzer};
use reqwest::blocking;
use std::*;

pub const APP_NAME: &str = "git-retrospective";
pub type DynErrResult<T> = Result<T, Box<dyn error::Error>>;

fn main() {
    match Command::parse_args() {
        Command::Help => {
            log::message(Command::Help.help());
        },
        Command::Config => {
            if let Err(e) = configure().and_then(|cfg| cfg.store()) {
                log::error(e.to_string());
            }
        }
        Command::Run => {
            if let Err(e) = run() {
                log::error(e.to_string());
            }
        }
        Command::RunD => {
            if let Err(e) = cli::rundaemon::run() {
                log::error(e.to_string());
            }
        }
        Command::InstallD => match launchd::install_daemon() {
            Err(e) => log::error(e.to_string()),
            Ok(path) => log::message(format!(
                r"Launch agent installed in {}
            Restart or logout is required in order for it to take effect",
                path
            )),
        },
        Command::Invalid => {
            log::error("Invalid command, try --help option");
        }
    };
}

fn run() -> DynErrResult<blocking::Response> {
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
    message.send_to_slack(app_config.slack_web_hook, message.format_slack()).map_err(Box::from)
}

