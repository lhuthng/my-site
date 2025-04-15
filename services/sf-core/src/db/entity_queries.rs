use sqlx::{Transaction, Postgres};
use crate::models::EntityType;

pub async fn create_entity(
    tx: &mut Transaction<'_, Postgres>,
    kind: EntityType,
) -> Result<i32, sqlx::Error> {
    let entity_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO entities (kind)
        VALUES ($1)
        RETURNING id
        "#,
        kind as i32,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(entity_id)
}

pub async fn attach_attribute(
    tx: &mut Transaction<'_, Postgres>,
    entity_id: i32,
    int_points: i32,
    str_points: i32,
    dex_points: i32,
    con_points: i32,
    lck_points: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO attributes (entity_id, int_points, str_points, dex_points, con_points, lck_points)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        entity_id,
        int_points,
        str_points,
        dex_points,
        con_points,
        lck_points,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(())
}