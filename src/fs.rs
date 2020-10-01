use crate::DynErrResult;
use directories::UserDirs;
use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum FsError {
    UserHome,
}

impl Error for FsError {}

impl Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsError::UserHome => write!(f, "HOME path is invalid"),
        }
    }
}

pub fn get_savedata_file() -> DynErrResult<PathBuf> {
    #[cfg(feature = "production")]
    let name = "savedata";
    #[cfg(not(feature = "production"))]
    let name = "savedata_dev";
    get_config_file(name)
}

pub fn get_config_file(name: impl AsRef<str>) -> DynErrResult<PathBuf> {
    let dirs = UserDirs::new().ok_or(FsError::UserHome)?;
    let path = dirs.home_dir();
    let mut path_buf = path.to_path_buf();
    path_buf.push(".config");
    path_buf.push(super::APP_NAME);
    path_buf.push(name.as_ref());
    path_buf.set_extension("toml");
    Ok(path_buf)
}

pub fn get_launch_agent_file(name: impl AsRef<str>) -> DynErrResult<PathBuf> {
    let dirs = UserDirs::new().ok_or(FsError::UserHome)?;
    let mut path_buf = dirs.home_dir().to_path_buf();
    path_buf.push(format!(
        "Library/LaunchAgents/{}.plist",
        name.as_ref()
    ));
    Ok(path_buf)
}
