use diesel::{select, sql_function, QueryableByName, RunQueryDsl};

use crate::args::AddCommand;
use crate::commands::CommandExecutor;
use crate::models::error::Error;
use crate::models::todo::TodoItem;
use crate::utils::date_util;
use crate::utils::db_util;

pub(crate) struct AddCommandExecutor {
    add_args: AddCommand,
}

impl AddCommandExecutor {
    pub(crate) fn new(add_args: AddCommand) -> Self {
        AddCommandExecutor { add_args }
    }
}

sql_function!(fn last_insert_rowid() -> Integer);

impl CommandExecutor for AddCommandExecutor {
    fn execute(&self) -> Result<(), Error> {
        println!("add_args: {:?}", self.add_args);
        if self.add_args.title.is_empty() {
            return Err(Error::new("Title cannot be empty".to_string()));
        }
        let mut todo_item = TodoItem::new(self.add_args.title.clone())?;
        let description = self.add_args.description.as_ref();
        if description.is_some() && !description.unwrap().is_empty() {
            todo_item.set_description(Some(description.unwrap().clone()));
        }
        let due_date = self.add_args.due_date.as_ref();
        if due_date.is_some() && !due_date.unwrap().is_empty() {
            let due_date_time = date_util::due_date_to_naive_date(due_date.unwrap())?;
            todo_item.set_due_date(Some(due_date_time));
        }
        // save to db
        let connection = &mut db_util::establish_connection();
        diesel::insert_into(crate::schema::todo_table::table)
            .values(&todo_item)
            .execute(connection)
            .map_err(|e| Error::new(e.to_string()))?;

        // 获取最后插入行的ID
        // let connection = &mut db_util::establish_connection();
        let last_id: i32 = select(last_insert_rowid())
            .get_result(connection)
            .map_err(|e| Error::new(e.to_string()))?;

        println!("Added todo item with id: {}", last_id);
        Ok(())
    }
}
