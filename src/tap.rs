use super::file::{FileTypes, Filename};
use super::local::share_path;
use super::{print_error_and_quit, warn_user};
use std::fs;
use std::path::Path;

pub fn shortcut(given: &str) -> FileTypes {
    match given {
        "mit" => FileTypes::MitLicense,
        "gpl" | "gpl3" | "gplv3" => FileTypes::Gplv3License,
        "py" => FileTypes::Python,
        "pyarg" => FileTypes::PythonArg,
        "latexmath" => FileTypes::LatexMathHomework,
        "latex" => FileTypes::Latex,
        "mainc" => FileTypes::MainC,
        "makewall" => FileTypes::MakeWall,
        _ => {
            list();
            print!("\n");
            print_error_and_quit!(format!("Could not find file '{}'", given));
        }
    }
}

pub fn list() {
    println!(
        "mit,
gpl, gpl3, gplv3,
py,
pyarg,
latexmath,
latex,
makewall,
mainc"
    );
}

pub fn create(given: String, force: bool, output: Option<String>) {
    let file_type = shortcut(given.as_str());

    let outname;
    if let Some(o) = output {
        outname = o;
    } else {
        outname = file_type.outname().to_string();
    }

    let uniquename = file_type.uniquename();

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
