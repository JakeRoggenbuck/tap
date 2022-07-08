use super::file::{FileTypes, Filename};
use super::local::share_path;
use super::{warn_user, print_error};
use std::fs;
use std::path::Path;

pub fn shortcut(given: &str) -> FileTypes {
    match given {
        "mit" => FileTypes::MitLicense,
        "gpl" | "gpl3" | "gplv3" => FileTypes::Gplv3License,
        "py" => FileTypes::Python,
        "pyarg" => FileTypes::PythonArg,
        _ => print_error!(format!("Could not find file '{}'", given)),
    }
}

pub fn create(given: &str, force: bool) {
    let file_type = shortcut(given);
    let outname = file_type.outname();
    let uniquename = file_type.uniquename();

    let uniquepath = Path::new(&share_path()).join(uniquename);

    match fs::copy(uniquepath, format!("./{}", outname)) {
        Ok(_) => println!("HEY"),
        Err(_) => eprintln!("POKE"),
    };

    warn_user!(format!("{} -> {}", uniquename, outname));
}
