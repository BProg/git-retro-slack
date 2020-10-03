use crate::{config::Config, DynErrResult};
use std::{env, io};

pub mod log;
pub mod rundaemon;

pub enum Command {
    Config,
    Run,
    RunD,
    InstallD,
    Invalid,
    Help,
}

impl Command {
    pub fn parse_args() -> Command {
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

    pub fn help(&self) -> String {
        match self {
            Command::Config => "allows to configure the slack hook, and repo path".into(),
            Command::Run => "runs the program".into(),
            Command::RunD => "it's designed to be used by the launch agent (daemon)".into(),
            Command::InstallD => "installs the launch agent parameters in user's space".into(),
            Command::Help => format!(
                r#"
gitretro v0.1.0

COMMANDS
run         {}
rund        {}
installd    {}
config      {}
help        {}
"#,
                Command::Run.help(),
                Command::RunD.help(),
                Command::InstallD.help(),
                Command::Config.help(),
                "prints this message"
            ),
            Command::Invalid => String::new(),
        }
    }
}


pub fn configure() -> DynErrResult<Config> {
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
