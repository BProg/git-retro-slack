mod cli;
mod config;
mod environment;
mod git;
mod launchd;
mod message;
mod printer;

use cli::{configure, get_command, Command};
use reqwest::Client;
use std::error::Error;

pub const APP_NAME: &str = "git-retrospective";

#[tokio::main]
async fn main() {
    match get_command() {
        Command::Config => {
            if let Err(e) = configure().and_then(|cfg| config::store_config(&cfg)) {
                print_error(e);
            }
        }
        Command::Run => {
            if let Err(e) = run().await {
                print_error(e);
            }
        }
        Command::InstallD => {
            match launchd::install_daemon() {
                Err(e) => print_error(e),
                Ok(path) => printer::print_launch_agent_installed(&path),
            }
        }
        Command::Invalid => {
            printer::print_invalid_command();
        }
    };
}

async fn run() -> Result<reqwest::Response, Box<dyn Error>> {
    let app_config = config::get_config();
    match app_config {
        Err(e) => Err(e),
        Ok(app_config) => {
            printer::print_config(&app_config);
            let repo = git::GitRepo::new(&app_config.repo_path);
            match repo.get_log() {
                Err(e) => Err(e),
                Ok(log) => send_to_slack(&app_config.slack_web_hook, &message::prettify(&log))
                    .await
                    .map_err(|e| Box::from(e)),
            }
        }
    }
}

fn print_error(error: Box<dyn ::std::error::Error>) {
    println!("error: {}", error)
}

async fn send_to_slack(hook: &str, log: &String) -> Result<reqwest::Response, reqwest::Error> {
    printer::print_slack_message(log);
    let client = Client::new();
    client
        .post(hook)
        .body(format!("{{\"text\": \"{}\"}}", log))
        .send()
        .await
}
