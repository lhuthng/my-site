use sqlx::{PgPool, Error};
use sqlx::migrate;
use tokio::fs;
use tokio::io;
use tokio::fs::DirEntry;

pub async fn Connect(uri: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(uri).await?;
    let mut entries = fs::read_dir("./migrations").await?;
    while let Some(entry) = entries.next_entry().await? {
        println!("{}", entry.path().display());
    }
    migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}