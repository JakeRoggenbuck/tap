pub fn shortcut(given: &str) -> &str {
    match given {
        "mit" => "MIT_LICENSE",
        "gpl" | "gpl3" | "gplv3" => "GPLV3_LICENSE",
        "c" => "COPYRIGHT",
        "py" => "PYTHON",
        "pyarg" => "PYTHON_ARG",
        _ => "NONE",
    }
}

pub fn create(given: &str, force: bool) {
    let name = shortcut(given);
    println!("{}", name);
}
