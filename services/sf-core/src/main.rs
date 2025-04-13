use std::env;
use dotenvy::dotenv;

mod db;
mod server;
mod services;
mod proto;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_database = env::var("DB_DATABASE").expect("DB_DATABASE must be set");
    let service_host = env::var("SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| "50051".to_string());

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_username, db_password, db_host, db_port, db_database
    );

    // match db::Connect(&db_url).await {
    //     Ok(_) => {
    //         println!("Connected to the database and migrations applied!");
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to connect to the database: {}", e);
    //         return;
    //     }
    // }

    let service_url = format!("{}:{}", service_host, service_port);
    println!("Starting gRPC server on {}", service_url);

    if let Err(e) = server::start().await {
        eprintln!("Failed to start the gRPC server: {}", e);
    }
}