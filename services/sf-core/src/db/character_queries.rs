use sqlx::{Transaction, Postgres};
use crate::models::{
    CharacterClass,
    ResourceType,
    EntityType,
    ContainerType,
};
use uuid::Uuid;
use crate::db::{
    entity_queries,
    container_queries,
};

async fn create_simple_character(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    entity_id: i32,
    job: CharacterClass,
    name: &str,
    level: i32,
    exp: i32,
) -> Result<Uuid, sqlx::Error> {
    let character_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO characters (user_id, entity_id, job, name, level, exp)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        user_id,
        entity_id,
        job as _,
        name,
        level,
        exp,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(character_id)
}

pub async fn create_character(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    job: CharacterClass,
    name: &str,
    level: i32,
    exp: i32,
) -> Result<Uuid, sqlx::Error> {
    let entity_id: i32 = entity_queries::create_entity(tx, EntityType::Character).await?;
    entity_queries::attach_attribute(tx, entity_id, 1, 1, 1, 1, 1).await?;
    let character_id: Uuid = create_simple_character(
        tx,
        user_id,
        entity_id,
        job,
        name,
        level,
        exp,
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO resources (character_id, kind, amount)
        VALUES 
            ($1, $2, $3),
            ($1, $4, $5)
        "#,
        character_id,
        ResourceType::Gold as _, 0,
        ResourceType::Mushroom as _, 0,
    )
    .fetch_one(&mut **tx)
    .await?;

    container_queries::create_container(tx, character_id, ContainerType::Inventory, 5).await?;
    container_queries::create_container(tx, character_id, ContainerType::Shop, 6).await?;

    Ok(character_id)
}