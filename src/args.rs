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
    /// Adds a new to-do
    Add(AddCommand),
}

#[derive(Parser, Debug)]
pub(crate) struct ListCommand {
    #[arg(short, long, default_value = "false")]
    all: bool,
}

#[derive(Parser, Debug)]
pub(crate) struct AddCommand {
    #[arg(short, long, help = "The title of the todo")]
    pub(crate) title: String,
    #[arg(short, long, help = "The description of the todo")]
    pub(crate) description: Option<String>,
    #[arg(short = 'u', long, help = "The due date of the todo YYYY-MM-DD")]
    pub(crate) due_date: Option<String>,
}
