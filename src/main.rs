use log::debug;
use structopt::StructOpt;

use tap::list;
use local::{
    create_local_config_dir, create_local_share, local_config_dir_exists, local_share_exists,
};
use tap::create;

pub mod display;
pub mod file;
pub mod local;
pub mod tap;

#[derive(StructOpt)]
#[structopt(name = "tap", about = "Quickly tap basic file into existence")]
enum Command {
    #[structopt(name = "it")]
    It {
        given: String,

        #[structopt(short, long)]
        force: bool,
    },

    #[structopt(name = "list")]
    List {},
}

fn main() {
    env_logger::init();

    if !local_config_dir_exists() {
        create_local_config_dir();
    }

    if !local_share_exists() {
        create_local_share();
    }

    match Command::from_args() {
        Command::It { given, force } => create(given.as_str(), force),
        Command::List {} => list(),
    }
}
