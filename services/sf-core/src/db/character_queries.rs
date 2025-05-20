use sqlx::{Transaction, Postgres};
use crate::models::{
    EntityType,
    ContainerType,
    Appearance,
    LookUpValue,
};
use uuid::Uuid;
use crate::db::{
    entity_queries,
    container_queries,
};
use crate::models::{
    Attribute,
};

async fn create_simple_character(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    entity_id: i32,
    job_id: i16,
    name: &str,
    level: i16,
    exp: i32,
) -> Result<Uuid, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding a character.");

    let character_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO characters (user_id, entity_id, job_id, name, level, exp)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        user_id,
        entity_id,
        job_id,
        name,
        level,
        exp,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(character_id)
}

pub async fn attach_resources(
    tx: &mut Transaction<'_, Postgres>,
    character_id: &Uuid,
) -> Result<(), sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding resources.");

    sqlx::query!(
        r#"
        INSERT INTO character_resources (character_id, resource_id)
        SELECT $1, id FROM resources
        "#,
        character_id,

    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn attach_appearance(
    tx: &mut Transaction<'_, Postgres>,
    entity_id: i32,
    appearance: &Appearance,
) -> Result<(), sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an appearance.");

    sqlx::query!(
        r#"
        INSERT INTO appearances 
        (entity_id, race_id, gender_id, hair, hair_color, beard, mouth, eyebrows, nose, ears, extra)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        entity_id,
        appearance.race_id,
        appearance.gender_id,
        appearance.hair,
        appearance.hair_color,
        appearance.beard,
        appearance.mouth,
        appearance.eyebrows,
        appearance.nose,
        appearance.ears,
        appearance.extra,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn attach_containers(
    tx: &mut Transaction<'_, Postgres>,
    character_id: &Uuid,
) -> Result<(), sqlx::Error> {

    container_queries::create_container(tx, character_id, ContainerType::Inventory, 5).await?;
    container_queries::create_container(tx, character_id, ContainerType::MagicShop, 6).await?;
    container_queries::create_container(tx, character_id, ContainerType::GearShop, 6).await?;

    Ok(())
}

pub async fn create_character(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    job_id: i16,
    name: &str,
    appearance: &Appearance,
    level: i16,
    exp: i32,
) -> Result<Uuid, sqlx::Error> {
    let entity_id: i32 = entity_queries::create_entity(tx, EntityType::Character).await?;
    entity_queries::attach_attributes(tx, Attribute { 
        entity_id: entity_id, 
        int: 1, 
        str: 1, 
        dex: 1, 
        con: 1, 
        lck: 1 
    }, None).await?;
    let character_id: Uuid = create_simple_character(
        tx,
        user_id,
        entity_id,
        job_id,
        name,
        level,
        exp,
    ).await?;

    attach_resources(tx, &character_id).await?;
    attach_containers(tx, &character_id).await?;

    Ok(character_id)
}