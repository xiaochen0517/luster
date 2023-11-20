use std::env;

use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "luster.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
