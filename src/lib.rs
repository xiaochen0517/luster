use clap::Parser;

use args::Args;
use config::Config;

use crate::args::Command;

mod args;
mod config;

pub fn run() {
    let _config = Config::read_config();

    println!("\n\n=================================================");
    let args = Args::parse();
    match args.command {
        Command::List(list_args) => {
            println!("list_args: {:?}", list_args);
        }
    }
}
