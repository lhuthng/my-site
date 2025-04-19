use sqlx::{Transaction, Postgres};
use crate::models::{
    ItemType,
    ItemTier,
    CharacterClass,
    Attribute,
    EntityType,
};
use crate::db::{
    entity_queries,
}

pub async fn create_item(
    tx: &mut Transaction<'_, Postgres>,
    variant_id: i32,
    kind: ItemType,
    tier: ItemTier,
    description: &str,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an item")

    let item_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO items (variant_id, kind, tier, description, price)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        name,
        kind as _,
        tier as _,
        description,
        price,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(item_id)
}

pub async fn create_equipable_item(
    tx: &mut Transaction<'_, Postgres>,
    variant_id: i32,
    kind: ItemType,
    tier: ItemTier,
    description: &str,
    price: i32,
    job: CharacterClass,
    attrs: Attribute, 
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an equipable item");

    let entity_id: i32 = entity_queries::create_entity(tx, EntityType::Item).await?;
    entity_queries::attach_attributes(tx, entity_id).await?;

    let item_id: i32 = create_item(
        tx,
        variant_id,
        item_type,
        tier,
        description,
        price,
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO equipable_items (item_id, entity_id, job)
        VALUES ($1, $2, $3)
        "#,
        item_id,
        entity_id,
        job,
    )
    .execute(&mut **tx)
    .await?;

    Ok(item_id)
}

pub async fn create_armor_item(
    tx: &mut Transaction<'_, Postgres>,
    variant_id: i32,
    kind: ItemType,
    tier: ItemTier,
    description: &str,
    price: i32,
    job: CharacterClass,
    attrs: Attribute, 
    armor_points: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an armor item.");

    let item_id: i32 = create_equipable_item(
        tx,
        variant_id,
        kind,
        tier,
        description,
        price,
        job,
        attrs
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO armor_items (item_id, armor_points)
        VALUES ($1, $2)
        "#,
        item_id,
        armor_points
    )
    .execute(&mut **tx)
    .await?;

    Ok(item_id)
}
