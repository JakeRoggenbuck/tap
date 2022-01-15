use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tap", about = " Quickly tap basic file into existence")]
enum Command {
    #[structopt(name = "it")]
    It {},
}

fn main() {
    env_logger::init();

    match Command::from_args() {
        Command::It {} => println!("Hello world!"),
    }
}
