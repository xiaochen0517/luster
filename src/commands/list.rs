use crate::args::ListCommand;
use crate::commands::CommandExecutor;
use crate::models::error::Error;

pub(crate) struct ListCommandExecutor {
    list_args: ListCommand,
}

impl ListCommandExecutor {
    pub(crate) fn new(list_args: ListCommand) -> Self {
        ListCommandExecutor { list_args }
    }
}

impl CommandExecutor for ListCommandExecutor {
    fn execute(&self) -> Result<(), Error> {
        println!("list_args: {:?}", self.list_args);
        Ok(())
    }
}
