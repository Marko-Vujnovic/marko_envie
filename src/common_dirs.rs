use crate as envie; use self::envie::*;

pub fn home_folder() -> std::path::PathBuf { let mut dir = std::path::PathBuf::new(); let username = os_username(); dir.push(format!("/home/{}", &username)); dir }
pub fn app_config_folder() -> std::path::PathBuf { home_folder().join(".config").join(&PROGRAM_INFO.name) }
pub fn app_cache_folder() -> std::path::PathBuf { home_folder().join(".cache").join(&PROGRAM_INFO.name) }
pub fn cwd() -> std::path::PathBuf { std::env::current_dir().unwrap() }