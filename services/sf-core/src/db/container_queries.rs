use sqlx::{Transaction, Postgres};
use crate::models::ContainerType;
use uuid::Uuid;

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