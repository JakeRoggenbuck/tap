use std::fs;
use super::local::share_path;
use std::path::Path;

pub fn shortcut(given: &str) -> &str {
    match given {
        "mit" => "MIT_LICENSE",
        "gpl" | "gpl3" | "gplv3" => "GPLV3_LICENSE",
        "py" => "PYTHON",
        "pyarg" => "PYTHON_ARG",
        _ => "NONE",
    }
}

pub fn create(given: &str, force: bool) {
    let name = shortcut(given);

    let filepath = Path::new(&share_path()).join(name);

    match fs::copy(filepath, format!("./{}", name)) {
        Ok(_) => println!("HEY"),
        Err(_) => eprintln!("POKE"),
    };

    println!("{}", name);
}
