mod cli;
mod config;
mod environment;
mod git;
mod launchd;
mod slack;

use git::RepoAnalyzer;
use cli::{configure, get_command, log, Command};
use reqwest::blocking;
use std::*;

pub const APP_NAME: &str = "git-retrospective";

fn main() {
    match get_command() {
        Command::Help => {
            print_usage();
        }
        Command::Config => {
            if let Err(e) = configure().and_then(|cfg| config::store_config(&cfg)) {
                log::error(e.to_string());
            }
        }
        Command::Run => {
            if let Err(e) = run() {
                log::error(e.to_string());
            }
        }
        Command::RunD => {
            if let Err(e) = run() {
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

pub(crate) fn print_usage() {
    let usage = r"gitretro v0.1.0

Arguments:
gitretro run            runs the program
gitretro rund           it's designed to be used by the launch agent (daemon)
gitretro installd       installs the launch agent parameters in user's space
gitretro config         allows to configure the slack hook, and repo path

Options:
--help                  prints this message";
    println!("{}", usage);
}

fn run() -> Result<blocking::Response, Box<dyn error::Error>> {
    let app_config = config::get_config()?;
    log::multiple(vec![
        log::Style::Message("Config: "),
        log::Style::Important(&app_config.to_string()),
    ]);
    let repo = RepoAnalyzer::new(&app_config.repo_path);
    let commits = repo.get_commits()?;
    let branches = repo.get_in_progress()?;
    let message = slack::message::create_message(commits, branches);
    send_to_slack(app_config.slack_web_hook, message).map_err(Box::from)
}

fn send_to_slack<T: AsRef<str>>(hook: T, log: T) -> reqwest::Result<blocking::Response> {
    log::message(format!("Sending to slack \n{}", log.as_ref()));
    let client = blocking::Client::new();
    client
        .post(hook.as_ref())
        .body(format!("{{\"text\": \"{}\"}}", log.as_ref()))
        .send()
}
