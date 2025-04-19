use sqlx::{Transaction, Postgres};
use crate::models::EntityType;

pub async fn create_entity(
    tx: &mut Transaction<'_, Postgres>,
    kind: EntityType,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an entity.");

    let entity_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO entities (kind)
        VALUES ($1)
        RETURNING id
        "#,
        kind as _,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(entity_id)
}

pub async fn attach_attributes(
    tx: &mut Transaction<'_, Postgres>,
    entity_id: i32,
    int: i32,
    str_: i32,
    dex: i32,
    con: i32,
    lck: i32,
) -> Result<(), sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an attribute.");

    sqlx::query!(
        r#"
        INSERT INTO attributes (entity_id, int, str, dex, con, lck)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        entity_id,
        int,
        str_,
        dex,
        con,
        lck,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}