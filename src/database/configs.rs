use std::env;

use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;

pub fn get_connection_mylsql() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must bet set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
