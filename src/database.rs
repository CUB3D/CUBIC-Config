use diesel::{MysqlConnection, Connection};
use std::env;

pub fn start_db_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Unable to connect to {}", database_url))
}
