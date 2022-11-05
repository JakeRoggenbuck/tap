use super::file::get_files;
use super::local::share_path;
use super::{print_error_and_quit, warn_user};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn shortcut(given: &str) -> (&str, &str) {
    let files = get_files();
    match files.get(given) {
        Some(a) => (given, a),
        None => {
            list(files);
            print_error_and_quit!("Alias not found.");
        }
    }
}

pub fn list(files: HashMap<&str, &str>) {
    for (k, v) in files {
        println!("{} -> {}", k, v);
    }
}

pub fn create(given: String, force: bool, output: Option<String>) {
    let pair = shortcut(given.as_str());

    let outname;
    if let Some(o) = output {
        outname = o;
    } else {
        outname = pair.1.to_string();
    }

    let uniquename = pair.0.to_string();

    let uniquepath = Path::new(&share_path()).join(uniquename);
    let local_path = format!("./{}", outname);

    if !force {
        if Path::new(&local_path).exists() {
            print_error_and_quit!("Already exists");
        }
    }

    match fs::copy(uniquepath.clone(), local_path.clone()) {
        Ok(_) => println!(
            "Wrote '{}' to '{}'",
            uniquepath.as_path().display(),
            local_path
        ),
        Err(_) => warn_user!(format!(
            "Could not copy '{}' to '{}'",
            uniquepath.as_path().display(),
            local_path
        )),
    };
}
