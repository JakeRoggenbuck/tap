#[macro_export]
macro_rules! warn_user {
    ($a:expr) => {{
        use colored::Colorize;
        println!("{}: {}", "WARN".bold().yellow(), $a,);
    }};
}

#[macro_export]
macro_rules! print_error_and_quit {
    ($a:expr) => {{
        use colored::Colorize;
        use std::process::exit;
        println!("{}: {}", "ERROR".bold().red(), $a,);
        exit(-1);
    }};
}
