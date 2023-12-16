use diesel::{QueryDsl, RunQueryDsl};

use crate::args::DoneCommand;
use crate::models::error::Error;
use crate::models::todo::TodoItemWithId;
use crate::schema::todo_table::dsl::*;
use crate::utils::db_util;

pub(crate) struct DoneCommandExecutor {
    args: DoneCommand,
}

impl DoneCommandExecutor {
    pub(crate) fn new(args: DoneCommand) -> Self {
        Self { args }
    }

    pub(crate) fn execute(&self) -> Result<(), Error> {
        println!("Done command executed");
        let connection = &mut db_util::establish_connection();
        let mut todo_info:TodoItemWithId = todo_table.find(self.args.id).first(connection)
            .map_err(|e| Error::new(e.to_string()))?;
        todo_info.completed = true;
        diesel::update(todo_table.find(self.args.id))
            .set(&todo_info)
            .execute(connection)
            .map_err(|e| Error::new(e.to_string()))?;
        println!("🎉Success: Completed todo item with id: {}, name: {}", self.args.id, todo_info.title);
        Ok(())
    }
}