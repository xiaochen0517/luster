use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::prelude::*;

use crate::models::error::Error;

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::todo_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct TodoItem {
    pub(crate) id: Option<i32>,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) due_date: Option<NaiveDate>,
    pub(crate) completed: bool,
    pub(crate) create_time: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::todo_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct TodoItemWithId {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) due_date: Option<NaiveDate>,
    pub(crate) completed: bool,
    pub(crate) create_time: NaiveDateTime,
}

impl TodoItem {
    pub(crate) fn new(title: String) -> Result<Self, Error> {
        Ok(TodoItem {
            id: None,
            title,
            description: None,
            due_date: None,
            completed: false,
            create_time: Local::now().naive_local(),
        })
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
    pub fn set_due_date(&mut self, due_date: Option<NaiveDate>) {
        self.due_date = due_date;
    }
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
    pub fn set_create_time(&mut self, create_time: NaiveDateTime) {
        self.create_time = create_time;
    }
}
