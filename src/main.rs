use structopt::StructOpt;
use log::debug;

use local::{create_local_config_dir, local_config_dir_exists};

pub mod config;
pub mod display;
pub mod local;

#[derive(StructOpt)]
#[structopt(name = "tap", about = "Quickly tap basic file into existence")]
enum Command {
    #[structopt(name = "it")]
    It {},
}

fn main() {
    env_logger::init();

     if !local_config_dir_exists() {
        create_local_config_dir();
    }

    match Command::from_args() {
        Command::It {} => println!("Hello world!"),
    }
}
