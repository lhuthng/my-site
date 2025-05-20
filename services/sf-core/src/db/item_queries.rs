use sqlx::{Transaction, Postgres};
use crate::models::{
    Attribute,
    EntityType,
};

use crate::db::{
    entity_queries,
};

pub async fn create_item(
    tx: &mut Transaction<'_, Postgres>,
    preset_item_id: i16,
    overridden_item_tier_id: Option<i16>,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an item");

    let item_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO items (preset_item_id, overridden_item_tier_id, price)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        preset_item_id,
        overridden_item_tier_id,
        price
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(item_id)
}

pub async fn create_equipable_item(
    tx: &mut Transaction<'_, Postgres>,
    attr: Attribute,
    preset_item_id: i16,
    overridden_item_tier_id: Option<i16>,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an equipable item");

    let entity_id: i32 = entity_queries::create_entity(tx, EntityType::Item).await?;
    entity_queries::attach_attributes(tx, attr, Some(entity_id)).await?;

    let item_id: i32 = create_item(
        tx,
        preset_item_id,
        overridden_item_tier_id,
        price,
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO equipable_items (item_id, entity_id)
        VALUES ($1, $2)
        "#,
        item_id,
        entity_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(item_id)
}

pub async fn create_armor_item(
    tx: &mut Transaction<'_, Postgres>,
    armor_points: i32,
    attr: Attribute,
    preset_item_id: i16,
    overridden_item_tier_id: Option<i16>,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding an armor item.");

    let item_id: i32 = create_equipable_item(
        tx,
        attr,
        preset_item_id,
        overridden_item_tier_id,
        price,
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

pub async fn create_weapon_item(
    tx: &mut Transaction<'_, Postgres>,
    damage: (i32, i32),
    attr: Attribute,
    preset_item_id: i16,
    overridden_item_tier_id: Option<i16>,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding a weapon item.");

    let item_id: i32 = create_equipable_item(
        tx,
        attr,
        preset_item_id,
        overridden_item_tier_id,
        price,
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO weapon_items (item_id, min_damage, max_damage)
        VALUES ($1, $2, $3)
        "#,
        item_id,
        damage.0,
        damage.1
    )
    .execute(&mut **tx)
    .await?;

    Ok(item_id)
}

pub async fn create_shield_item(
    tx: &mut Transaction<'_, Postgres>,
    armor_points: i32,
    block_points: i32,
    attr: Attribute,
    preset_item_id: i16,
    overridden_item_tier_id: Option<i16>,
    price: i32,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding a weapon item.");

    let item_id: i32 = create_equipable_item(
        tx,
        attr,
        preset_item_id,
        overridden_item_tier_id,
        price,
    ).await?;

    sqlx::query!(
        r#"
        INSERT INTO shield_items (item_id, armor_points, block_points)
        VALUES ($1, $2, $3)
        "#,
        item_id,
        armor_points,
        block_points,
    )
    .execute(&mut **tx)
    .await?;

    Ok(item_id)
}