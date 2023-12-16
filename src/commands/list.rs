use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use prettytable::{row, Table};

use crate::args::ListCommand;
use crate::commands::CommandExecutor;
use crate::models::error::Error;
use crate::models::todo::TodoItemWithId;
use crate::schema::todo_table::{completed, description, title};
use crate::schema::todo_table::dsl::todo_table;
use crate::utils::db_util;

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
        let connection = &mut db_util::establish_connection();
        let mut select_statement = todo_table.into_boxed::<diesel::sqlite::Sqlite>();
        if !self.list_args.all {
            select_statement = select_statement.filter(completed.eq(false));
        }
        if self.list_args.title.is_some() && !self.list_args.title.as_ref().unwrap().is_empty() {
            select_statement = select_statement.filter(
                title.like(format!("%{}%", self.list_args.title.as_ref().unwrap())),
            );
        }
        if self.list_args.description.is_some()
            && !self.list_args.description.as_ref().unwrap().is_empty()
        {
            select_statement = select_statement.filter(
                description.like(format!(
                        "%{}%", self.list_args.description.as_ref().unwrap())),
            );
        }
        let todo_vec: Vec<TodoItemWithId> = select_statement
            .limit(self.list_args.size)
            .offset((self.list_args.page - 1) * self.list_args.size)
            .load(connection)
            .map_err(|e| Error::new(e.to_string()))?;
        Self::print_table(todo_vec);
        Ok(())
    }
}

impl ListCommandExecutor {
    fn print_table(todo_vec: Vec<TodoItemWithId>) {
        let mut table = Table::new();
        table.add_row(row![
            "ID",
            "Title",
            "Description",
            "Due Date",
            "Completed",
            "Created Time"
        ]);
        for todo_item in todo_vec {
            table.add_row(row![
                todo_item.id,
                todo_item.title,
                todo_item.description.unwrap_or("NULL".to_string()),
                match todo_item.due_date {
                    Some(due_date) => due_date.format("%Y-%m-%d").to_string(),
                    None => "NULL".to_string(),
                },
                todo_item.completed.to_string(),
                todo_item
                    .create_time
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            ]);
        }
        table.printstd();
    }
}
