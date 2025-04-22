use sqlx::{PgPool, Transaction, Postgres, Error};
use crate::models::LookUpValue;
use crate::models::LookUpValue as LookUpValueDB;
use crate::proto::sf_core::LookUpValue as LookUpValueGRPC;

pub mod user_queries;
pub mod character_queries;
pub mod entity_queries;
pub mod container_queries;
pub mod maintain_presets;

pub async fn connect(uri: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(uri).await?;
    Ok(pool)
}

impl From<LookUpValueDB> for LookUpValueGRPC {
    fn from(db: LookUpValueDB) -> Self {
        LookUpValueGRPC {
            id: db.id as i32,
            name: db.name,
        }
    }
}

pub async fn get_look_up_values(
    tx: &mut Transaction<'_, Postgres>,
    name: &str
) -> Result<Vec<LookUpValue>, sqlx::Error> {
    let query = format!("SELECT id, name FROM {}", name);
    let result: Vec<LookUpValue> = sqlx::query_as::<_, LookUpValue>(&query)
    .fetch_all(&mut **tx)
    .await?;

    Ok(result)
}