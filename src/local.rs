use super::debug;
use std::fs;
use std::path::Path;

pub fn check_for_local(path_name: &str) -> bool {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(format!("{}/{}/", path.display(), &path_name).as_str()).to_owned();

            if local_path.exists() {
                return true;
            } else {
                debug!("Could not find ~/{}", path_name);
            }
        }
        None => println!("Could not get home directory"),
    }

    return false;
}

pub fn create_local(path_name: &str) {
    match home::home_dir() {
        Some(path) => {
            let local_path =
                Path::new(&format!("{}/{}", path.display(), &path_name).as_str()).to_owned();

            match fs::create_dir_all(local_path.clone()) {
                Ok(_) => debug!("Created {:?}", local_path),
                Err(e) => eprintln!("{}", e),
            }
        }
        None => println!("Could not get home directory"),
    }
}

pub fn local_share_exists() -> bool {
    check_for_local(".local/share/tap/")
}

pub fn create_local_share() {
    create_local(".local/share/tap");
}

pub fn config_path() -> String {
    match home::home_dir() {
        Some(path) => {
            format!("{}/{}", path.display(), ".config/tap/tap.toml")
        }
        None => {
            println!("Could not get home directory");
            String::new()
        }
    }
}

pub fn local_config_file_exists() -> bool {
    Path::new(&config_path()).exists()
}

pub fn local_config_dir_exists() -> bool {
    check_for_local(".config/tap/")
}

pub fn create_local_config_dir() {
    create_local(".config/tap/")
}
