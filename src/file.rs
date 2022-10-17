pub enum FileTypes {
    MitLicense,
    Gplv3License,
    Python,
    PythonArg,
    LatexMathHomework,
    Latex,
    NoFile,
    MainC,
    MakeWall,
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
            FileTypes::LatexMathHomework => "main.tex",
            FileTypes::Latex => "main.tex",
            FileTypes::MainC => "main.c",
            FileTypes::MakeWall => "Makefile",
            FileTypes::NoFile => "NoFile",
        }
    }

    fn uniquename(&self) -> &str {
        match self {
            FileTypes::MitLicense => "MIT_LICENSE",
            FileTypes::Gplv3License => "GPLV3_LICENSE",
            FileTypes::Python => "PYTHON",
            FileTypes::PythonArg => "PYTHON_ARG",
            FileTypes::LatexMathHomework => "LATEX_MATH_HOMEWORK",
            FileTypes::Latex => "LATEX",
            FileTypes::MainC => "MAIN_C",
            FileTypes::MakeWall => "MAKE_WALL",
            FileTypes::NoFile => "NO_FILE",
        }
    }
}
