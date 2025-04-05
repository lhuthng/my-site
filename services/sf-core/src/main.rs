use sqlx::{PgPool, Error};
use std::env;
use dotenv::dotenv;

mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT").expect("DB_PORT must be set");
    let database = env::var("DB_DATABASE").expect("DB_DATABASE must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );

    match db::Connect(&database_url).await {
        Ok(pool) => {
            println!("Connected to the database and migrations applied!");
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
        }
    }
}