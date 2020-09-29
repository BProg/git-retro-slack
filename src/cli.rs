use crate::config::Config;
use std::{env, error::Error, io};

pub mod log;

pub enum Command {
    Config,
    Run,
    RunD,
    InstallD,
    Invalid,
    Help,
}

pub fn get_command() -> Command {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(command) => match &command[..] {
            "run" => Command::Run,
            "rund" => Command::RunD,
            "config" => Command::Config,
            "installd" => Command::InstallD,
            "--help" | "-h" => Command::Help,
            _ => Command::Invalid,
        },
        None => Command::Invalid,
    }
}

pub fn configure() -> Result<Config, Box<dyn Error>> {
    log::important("Repository absolute path:");
    let mut path = String::new();
    match io::stdin().read_line(&mut path) {
        Ok(bytes) => {
            if bytes == 0 {
                log::error("Repo path is not valid");
            }
        }
        Err(_) => println!("Failed to read input"),
    };
    let mut hook = String::new();
    log::important("Slack web hook:");
    match io::stdin().read_line(&mut hook) {
        Ok(bytes) => {
            if bytes == 0 {
                log::error("Slack hook is not valid");
            }
        }
        Err(_) => println!("Failed to read input"),
    };
    path = path.trim().into();
    hook = hook.trim().into();

    if !path.is_empty() && !hook.is_empty() {
        Ok(Config::new(&path, &hook))
    } else {
        Err("Failed to create a new config".into())
    }
}
