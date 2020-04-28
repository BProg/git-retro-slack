use directories::UserDirs;
use std::io::Write;
use std::{error::Error, fmt::Display};
use std::{fs::File};

mod parameters;

#[derive(Debug)]
pub enum DaemonError {
    UserHome,
    ExePath
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

pub fn install_daemon() -> Result<(), Box<dyn Error>> {
    let dirs = UserDirs::new().ok_or(DaemonError::UserHome)?;
    let mut path_buf = dirs.home_dir().to_path_buf();
    path_buf.push("Library");
    path_buf.push("LaunchAgents");
    path_buf.push("com.ionostafi.gitretro");
    path_buf.set_extension("plist");
    let mut file = File::create(path_buf.as_path())?;
    let data = create_launch_agent_plist_content()?;
    file.write(data.as_bytes());
    Ok(())
}

fn create_launch_agent_plist_content() -> Result<String, Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    match exe_path.to_str() {
        None => Err(DaemonError::ExePath.into()),
        Some(path) => {
            let launchd_params = parameters::create_parameters(path);
            Ok(launchd_params)
        }
    }
}
