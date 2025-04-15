use sqlx::{Transaction, Postgres};
use crate::models::{
    ContainerType,
};
use uuid::Uuid;

pub async fn create_container(
    tx: &mut Transaction<'_, Postgres>,
    character_id: Uuid,
    kind: ContainerType,
    capacity: i32,
) -> Result<i32, sqlx::Error> {
    let container_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO containers (character_id, kind, capacity)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        character_id as _,
        kind as _,
        capacity,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(container_id)
}