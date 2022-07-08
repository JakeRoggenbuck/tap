#[macro_export]
macro_rules! warn_user {
    ($a:expr) => {{
        use colored::Colorize;
        println!("{}: {}", "WARN".bold().yellow(), $a,);
    }};
}

#[macro_export]
macro_rules! print_error {
    ($a:expr) => {{
        use colored::Colorize;
        println!("{}: {}", "ERROR".bold().red(), $a,);
    }};
}
