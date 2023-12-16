use clap::Parser;
use diesel::migration::MigrationConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use args::Args;
use config::Config;

use crate::args::Command;
use crate::commands::add::AddCommandExecutor;
use crate::commands::CommandExecutor;
use crate::commands::done::DoneCommandExecutor;
use crate::commands::list::ListCommandExecutor;
use crate::utils::db_util;

mod args;
mod commands;
mod config;
mod models;
mod schema;
mod utils;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
pub fn run() {
    let _config = Config::read_config();

    let connection = &mut db_util::establish_connection();
    // 使用 diesel_migrations 进行数据库迁移
    connection.setup().expect("🚨Error: Setup database error");
    // 迁移数据库
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("🚨Error: Run migrations error");

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
