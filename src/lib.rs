use clap::Parser;

use args::Args;
use config::Config;

use crate::args::Command;
use crate::commands::add::AddCommandExecutor;
use crate::commands::list::ListCommandExecutor;
use crate::commands::CommandExecutor;
use crate::commands::done::DoneCommandExecutor;

mod args;
mod commands;
mod config;
mod models;
mod schema;
mod utils;

pub fn run() {
    let _config = Config::read_config();

    let args = Args::parse();
    let run_result = match args.command {
        Command::List(list_args) => ListCommandExecutor::new(list_args).execute(),
        Command::Add(add_args) => AddCommandExecutor::new(add_args).execute(), 
        Command::Done(done_args) => DoneCommandExecutor::new(done_args).execute(),
    };

    if run_result.is_err() {
        println!("🚨Error: {}", run_result.unwrap_err());
    }
}
