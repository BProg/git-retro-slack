mod cli;
mod config;
mod environment;
mod git;
mod launchd;
mod message;
mod printer;

use cli::{configure, get_command, Command};
use std::*;
use reqwest::{blocking};


pub const APP_NAME: &str = "git-retrospective";

fn main() {
    match get_command() {
        Command::Help => {
            printer::print_usage();
        }
        Command::Config => {
            if let Err(e) = configure().and_then(|cfg| config::store_config(&cfg)) {
                printer::print_error(e);
            }
        }
        Command::Run => {
            if let Err(e) = run() {
                printer::print_error(e);
            }
        }
        Command::InstallD => {
            match launchd::install_daemon() {
                Err(e) => printer::print_error(e),
                Ok(path) => printer::print_launch_agent_installed(&path),
            }
        }
        Command::Invalid => {
            printer::print_invalid_command();
        }
    };
}

fn run() -> Result<blocking::Response, Box<dyn error::Error>> {
    let app_config = config::get_config();
    match app_config {
        Err(e) => Err(e),
        Ok(app_config) => {
            printer::print_config(&app_config);
            let repo = git::GitRepo::new(&app_config.repo_path);
            match repo.get_log() {
                Err(e) => Err(e),
                Ok(log) => send_to_slack(&app_config.slack_web_hook, &message::prettify(&log))
                    .map_err(|e| Box::from(e)),
            }
        }
    }
}

fn send_to_slack(hook: &str, log: &String) -> reqwest::Result<blocking::Response> {
    printer::print_slack_message(log);
    let client = blocking::Client::new();
    client
        .post(hook)
        .body(format!("{{\"text\": \"{}\"}}", log))
        .send()
}
