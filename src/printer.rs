use crate::config::Config;
use chrono::NaiveDateTime;
use colorful::{Color, Colorful};

const COL_NOTICE: Color = Color::DarkGray;
const COL_ERROR: Color = Color::DarkRed1;

pub fn print_config(config: &Config) {
    let prefix = "Config:".color(COL_NOTICE);
    println!("{} {}", prefix, config.to_string().bold());
}

pub fn print_time_range(from: &NaiveDateTime, to: &NaiveDateTime) {
    let from_msg = "Searching logs from".color(COL_NOTICE);
    let to_msg = "to".color(COL_NOTICE);
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
    println!("{}", "Repo path is not valid".color(COL_ERROR).bold());
}

pub(crate) fn print_slack_hook_invalid() {
    println!("{}", "Slack hook is not valid".color(COL_ERROR).bold());
}

pub(crate) fn print_commit(commit: &git2::Commit) {
    println!(
        "Found commit {}",
        commit.summary().unwrap_or("Cannot read summary")
    );
}

pub(crate) fn print_slack_message(log: &String) {
    println!("Sending to slack\n{}", log);
}

pub(crate) fn print_invalid_command() {
    println!("{}", "Invalid command, try --help option".color(COL_ERROR));
}

pub(crate) fn print_launch_agent_installed(path: &str) {
    println!(
        r"Launch agent installed in {}
Restart or logout is required in order for it to take effect",
        path.color(COL_NOTICE)
    );
}

pub(crate) fn print_usage() {
    let usage = r"gitretro

Usage:
gitretro run            runs the program
gitretro installd       installs the launch agent parameters in user's space
gitretro config         configures the tool

Options:
--help          prints this message
";
    println!("{}", usage);
}
