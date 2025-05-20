use sqlx::{Transaction, Postgres};
use crate::models::ContainerType;
use uuid::Uuid;
use crate::proto::sf_core::{
    Slot, slot
};
use crate::models::{
    Shop,
    ResSlot,
};

pub async fn create_container(
    tx: &mut Transaction<'_, Postgres>,
    character_id: &Uuid,
    kind: ContainerType,
    capacity: i32,
) -> Result<i32, sqlx::Error> {
    
    #[cfg(debug_assertions)]
    println!("Adding a container.");

    let container_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO containers (character_id, kind, capacity)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        character_id,
        kind as _,
        capacity
    )
    .fetch_one(&mut **tx)
    .await?;

    match kind {
        ContainerType::MagicShop | ContainerType::GearShop => {
            #[cfg(debug_assertions)]
            println!("Extend with a refresh date.");

            sqlx::query!(
                r#"
                INSERT INTO shops (container_id)
                VALUES ($1)
                "#,
                container_id
            )
            .execute(&mut **tx)
            .await?;
        }
        _ => {}
    }

    Ok(container_id)
}

pub async fn get_shop(
    tx: &mut Transaction<'_, Postgres>,
    character_id: Uuid,
    kind: ContainerType,
) -> Result<Shop, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Getting a shop.");

    let shop = sqlx::query_as::<_, Shop>(
        r#"
        SELECT container_id, last_refresh, capacity, character_id
        FROM shops
        JOIN containers ON containers.id = shops.container_id
        WHERE containers.character_id = $1
            AND containers.kind = $2
        "#
    )
    .bind(character_id)
    .bind(kind)
    .fetch_one(&mut **tx)
    .await?;

    Ok(shop)
}

pub async fn get_items_from_gear_shop(
    tx: &mut Transaction<'_, Postgres>,
    character_id: Uuid,
    kind: ContainerType,
) -> Result<Vec<Slot>, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Getting items.");

    let res_slots = sqlx::query_as::<_, ResSlot>(
        r#"
        SELECT 
            loc.index AS index, 
            loc.item_id AS item_id, 
            pre.name AS name,
            pre.description AS description,
            pre.item_category_id AS item_category_id,
            pre.item_sub_category_id AS item_sub_category_id
        FROM item_locations AS loc
        JOIN containers AS con ON loc.container_id = con.id
        JOIN items AS it ON it.id = loc.item_id
        JOIN preset_items AS pre ON pre.id = it.preset_item_id
        WHERE 
            con.character_id = $1
            AND con.kind = $2;
        "#
    )
    .bind(character_id)
    .bind(kind)
    .fetch_all(&mut **tx).await?;
    
    let mut slots = Vec::<Slot>::new();
    for res_slot in res_slots {
        slots.push(Slot {
            index: res_slot.index,
            item_id: res_slot.item_id,
            item: get_item(tx, res_slot.item_id).await?
        })
    }
    
    Ok(slots)
}

pub async fn get_item(
    tx: &mut Transaction<'_, Postgres>,
    item_id: i32,
) -> Result<Option<slot::Item>, sqlx::Error> {
    Ok(None)
}

pub async fn refresh_shop(
    tx: &mut Transaction<'_, Postgres>,
    container_id: i32,
    kind: ContainerType,
) -> Result<(), sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Updating item's last refresh.");

    sqlx::query!(
        r#"
        UPDATE shops
        SET last_refresh = CURRENT_DATE
        WHERE container_id = $1
        "#,
        container_id
    ).execute(&mut **tx)
    .await?;

    Ok(())
}