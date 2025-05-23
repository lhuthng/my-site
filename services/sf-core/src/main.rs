use std::env;
use dotenvy::from_filename;
use tonic::Status;

mod db;
mod models;
mod server;
mod services;
mod proto;
mod misc;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        from_filename(".env.debug").ok();
    } else {
        from_filename(".env").ok();
    }

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

    let pool = match db::connect(&db_url).await {
        Ok(pool) => {
            println!("Trying to verify");
            match db::maintain_presets::verify_preset_items(&pool).await {
                Ok(_) => { println!("Verified"); }
                Err(e) => { eprintln!("Failed {}", e); }
            };
            match db::look_up_table_queries::initialize(&pool).await {
                Ok(_) => { println!("Initialized"); }
                Err(e) => { eprintln!("Failed {}", e); }
            };
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return;
        }
    };

    let service_url = format!("{}:{}", service_host, service_port);
    println!("Starting gRPC server on {}", service_url);

    if let Err(e) = server::start(&pool).await {
        eprintln!("Failed to start the gRPC server: {}", e);
    }
}