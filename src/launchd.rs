use std::fs::File;
use std::io::Write;
use std::{error::Error, fmt::Display};

use crate::{fs::get_launch_agent_file, DynErrResult};

mod parameters;

#[derive(Debug)]
pub enum DaemonError {
    ExePath,
}

impl Display for DaemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaemonError::ExePath => write!(f, "Executable path is invalid"),
        }
    }
}

impl Error for DaemonError {}

pub fn install_daemon() -> DynErrResult<String> {
    #[cfg(feature = "production")]
    let name = "com.ionostafi.gitretro";
    #[cfg(not(feature = "production"))]
    let name = "com.ionostafi.gitretro_dev";
    let path = get_launch_agent_file(name)?;
    let mut file = File::create(path.as_path())?;
    let data = create_launch_agent_plist_content(name)?;
    file.write_all(data.as_bytes())?;
    Ok(path.to_string_lossy().into())
}

fn create_launch_agent_plist_content(label: &str) -> DynErrResult<String> {
    let exe_path = std::env::current_exe()?;
    match exe_path.to_str() {
        None => Err(DaemonError::ExePath.into()),
        Some(path) => {
            let launchd_params = parameters::create_parameters(path, label);
            Ok(launchd_params)
        }
    }
}
