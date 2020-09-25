mod cli;
mod config;
mod environment;
mod git;
mod launchd;
mod message;

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
    let usage = r#"gitretro

                    Usage:
                    gitretro run            runs the program
                    gitretro installd       installs the launch agent parameters in user's space
                    gitretro config         configures the tool

                    Options:
                    --help          prints this message"#;
    println!("{}", usage);
}

fn run() -> Result<blocking::Response, Box<dyn error::Error>> {
    let app_config = config::get_config()?;
    log::multiple(vec![
        log::Style::Message("Config: "),
        log::Style::Important(&app_config.to_string()),
    ]);
    let repo = git::GitRepo::new(&app_config.repo_path);
    let log = repo.get_log()?;
    send_to_slack(&app_config.slack_web_hook, &message::prettify(&log)).map_err(Box::from)
}

fn send_to_slack(hook: &str, log: &str) -> reqwest::Result<blocking::Response> {
    log::message(format!("Sending to slack \n{}", log));
    let client = blocking::Client::new();
    client
        .post(hook)
        .body(format!("{{\"text\": \"{}\"}}", log))
        .send()
}
