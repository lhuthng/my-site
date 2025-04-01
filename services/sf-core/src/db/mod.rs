use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub fn establish_connection() -> PgConnection {

    let username = env::var("POSTGRESQL_USERNAME").expect("Missing POSTGRESQL_USERNAME");
    let password = env::var("POSTGRESQL_PASSWORD").expect("Missing POSTGRESQL_PASSWORD");
    let host = env::var("POSTGRESQL_HOST").expect("Missing POSTGRESQL_HOST");
    let port = env::var("POSTGRESQL_PORT").expect("Missing POSTGRESQL_PORT");
    let db = env::var("POSTGRESQL_DB").expect("Missing POSTGRESQL_DB");

    let database_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, db
    );

    PgConnection::establish(&database_uri)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_uri))
}