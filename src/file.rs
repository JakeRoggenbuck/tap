pub enum FileTypes {
    MitLicense,
    Gplv3License,
    Python,
    PythonArg,
    NoFile,
}

pub trait Filename {
    fn outname(&self) -> &str;
    fn uniquename(&self) -> &str;
}

impl Filename for FileTypes {
    fn outname(&self) -> &str {
        match self {
            FileTypes::MitLicense => "LICENSE",
            FileTypes::Gplv3License => "LICENSE",
            FileTypes::Python => "main.py",
            FileTypes::PythonArg => "main.py",
            FileTypes::NoFile => "NoFile",
        }
    }

    fn uniquename(&self) -> &str {
        match self {
            FileTypes::MitLicense => "MitLicense",
            FileTypes::Gplv3License => "Gplv3License",
            FileTypes::Python => "Python",
            FileTypes::PythonArg => "PythonArg",
            FileTypes::NoFile => "NoFile",
        }
    }
}
