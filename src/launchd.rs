use directories::UserDirs;
use std::fs::File;
use std::io::Write;
use std::{error::Error, fmt::Display};

mod parameters;

#[derive(Debug)]
pub enum DaemonError {
    UserHome,
    ExePath,
}

impl Display for DaemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaemonError::UserHome => write!(f, "HOME path is invalid"),
            DaemonError::ExePath => write!(f, "Executable path is invalid"),
        }
    }
}

impl Error for DaemonError {}

pub fn install_daemon() -> Result<String, Box<dyn Error>> {
    let dirs = UserDirs::new().ok_or(DaemonError::UserHome)?;
    let mut path_buf = dirs.home_dir().to_path_buf();
    path_buf.push(format!(
        "Library/LaunchAgents/{}.plist",
        crate::environment::get_launch_agent_file()
    ));
    let mut file = File::create(path_buf.as_path())?;
    let data = create_launch_agent_plist_content()?;
    file.write_all(data.as_bytes())?;
    Ok(path_buf.to_string_lossy().into())
}

fn create_launch_agent_plist_content() -> Result<String, Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    match exe_path.to_str() {
        None => Err(DaemonError::ExePath.into()),
        Some(path) => {
            let launchd_params =
                parameters::create_parameters(path, crate::environment::get_launch_agent_file());
            Ok(launchd_params)
        }
    }
}
