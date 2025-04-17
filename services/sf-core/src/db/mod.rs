use sqlx::{PgPool, Error};
use sqlx::migrate;
use tokio::fs;

pub mod user_queries;
pub mod character_queries;
pub mod entity_queries;
pub mod container_queries;

pub async fn connect(uri: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(uri).await?;
    // let mut entries = fs::read_dir("./migrations").await?;
    // while let Some(entry) = entries.next_entry().await? {
    //     println!("{}", entry.path().display());
    // }
    // migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
