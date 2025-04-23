use sqlx::{PgPool, Transaction, Postgres, Error};

pub mod user_queries;
pub mod character_queries;
pub mod entity_queries;
pub mod container_queries;
pub mod maintain_presets;
pub mod look_up_table_queries;

pub async fn connect(uri: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(uri).await?;
    Ok(pool)
}