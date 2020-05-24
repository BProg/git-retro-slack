pub fn get_config_file() -> &'static str {
    if cfg!(feature = "production") {
        "config"
    } else {
        "config_dev"
    }
}

pub fn get_launch_agent_file() -> &'static str {
    if cfg!(feature = "production") {
        "com.ionostafi.gitretro"
    } else {
        "com.ionostafi.gitretro_dev"
    }
}
