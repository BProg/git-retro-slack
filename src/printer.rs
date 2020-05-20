use crate::config::Config;
use chrono::NaiveDateTime;
use colorful::{Color, Colorful};

const PREFIX_COLOR: Color = Color::DarkGray;

pub fn print_config(config: &Config) {
    let prefix = "Config:".color(PREFIX_COLOR);
    println!("{} {}", prefix, config.to_string().bold());
}

pub fn print_time_range(from: &NaiveDateTime, to: &NaiveDateTime) {
    let from_msg = "Searching logs from".color(PREFIX_COLOR);
    let to_msg = "to".color(PREFIX_COLOR);
    println!(
        "{} {} {} {}",
        from_msg,
        from.to_string().bold(),
        to_msg,
        to.to_string().bold()
    );
}

pub fn ask_repo_path() {
    let question = "Repository absolute path:".bold();
    println!("{} ", question);
}

pub(crate) fn ask_slack_hook() {
    let question = "Slack web hook:".bold();
    println!("{} ", question);
}

pub(crate) fn print_repo_invalid() {
    println!("{}", "Repo path is not valid".color(Color::DarkRed1).bold());
}

pub(crate) fn print_slack_hook_invalid() {
    println!("{}", "Slack hook is not valid".color(Color::DarkRed1).bold());
}

pub(crate) fn print_commit(commit: &git2::Commit) {
    println!("Found commit {}", commit.summary().unwrap_or("Cannot read summary"));
}

pub(crate) fn print_slack_message(log: &String) {
    println!("Sending to slack\n{}", log);
}

pub(crate) fn print_invalid_command() {
    println!("{}", "Invalid command".color(Color::DarkRed1));
}
