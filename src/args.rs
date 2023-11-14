use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "luster")]
#[command(about = "A fictional version of the Git command", long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Lists all todos
    List(ListCommand),
}

#[derive(Parser, Debug)]
pub(crate) struct ListCommand {
    #[arg(short, long, default_value = "false")]
    all: bool,
}
