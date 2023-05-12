use std::collections::HashMap;

pub fn get_files() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("mit", "LICENSE"),
        ("gpl", "LICENSE"),
        ("py", "main.py"),
        ("pyarg", "main.py"),
        ("mathlatex", "main.tex"),
        ("latex", "main.tex"),
        ("mainc", "main.c"),
        ("makewall", "Makefile"),
        ("html", "index.html"),
        ("make", "Makefile"),
        ("maincpp", "main.cpp"),
    ])
}
