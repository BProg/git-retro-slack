use crate::printer;
use crate::config::Config;
use std::{error::Error, io, env};

pub enum Command {
    Config,
    Run,
    InstallD
}

pub fn get_command() -> Command {
    let mut args = env::args().into_iter().skip(1);
    match args.next() {
        Some(command) => {
            match &command[..] {
                "config" => Command::Config,
                "installd" => Command::InstallD,
                _ => Command::Run
            }
        },
        None => Command::Run
    }
}

pub fn configure() -> Result<Config, Box<dyn Error>> {
    printer::ask_repo_path();
    let mut path = String::new();
    match io::stdin().read_line(&mut path) {
        Ok(bytes) => {
            if bytes == 0 {
                printer::print_repo_invalid();
            }
        }
        Err(_) => println!("Failed to read input"),
    };
    let mut hook = String::new();
    printer::ask_slack_hook();
    match io::stdin().read_line(&mut hook) {
        Ok(bytes) => {
            if bytes == 0 {
                printer::print_slack_hook_invalid();
            }
        }
        Err(_) => println!("Failed to read input"),
    };
    path = path.trim().into();
    hook = hook.trim().into();

    if path.len() > 0 && hook.len() > 0 {
        Ok(Config::new(&path, &hook))
    } else {
        Err("Failed to create a new config".into())
    }
}
