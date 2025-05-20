use sqlx::{Transaction, Postgres};
use crate::models::{Attribute, EntityType};

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
    attr: Attribute,
    overridden_entity_id: Option<i32>
) -> Result<(), sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an attribute.");
    let entity_id = match overridden_entity_id {
        Some(val) => val,
        None => attr.entity_id
    };
    sqlx::query!(
        r#"
        INSERT INTO attributes (entity_id, int, str, dex, con, lck)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        entity_id,
        attr.int,
        attr.str,
        attr.dex,
        attr.con,
        attr.lck,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}