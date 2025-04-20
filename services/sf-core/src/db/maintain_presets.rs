use sqlx::{
    Transaction,
    Postgres,
    FromRow,
};
use serde::{
    Deserialize,
};
use std::error::Error;
use std::fs::File;
use tonic::Status;

#[derive(Debug, Deserialize, FromRow)]
struct Preset {
    pub name: String,
    pub tier: String,
    pub category: String,
    pub description: String,
}
use std::collections::HashMap;

pub async fn verify_preset_items(
    tx: &mut Transaction<'_, Postgres>
) -> Result<(), Status> {

    println!("hello?");

    #[cfg(debug_assertions)]
    println!("Verifying preset items");

    let db_presets: Vec<Preset> = sqlx::query_as::<_, Preset>(
        r#"
        SELECT pre.name AS name, tier.name AS tier, cat.name AS category, pre.description AS description
        FROM preset_items AS pre
        JOIN item_categories AS cat ON pre.item_category_id = cat.id
        JOIN item_tiers AS tier ON pre.item_tier_id = tier.id
        "#
    )
    .fetch_all(&mut **tx)
    .await.map_err(|e| {
        Status::internal(format!("DB error: {}", e))
    })?;

    let file = File::open("data/preset_items.csv").map_err(|e| {
        Status::internal(format!("CSV error: {}", e))
    })?;;
    let mut rdr = csv::Reader::from_reader(file);

    let csv_presets: Vec<Preset> = rdr
        .deserialize()
        .collect::<Result<_, _>>()
        .map_err(|e| {
            eprintln!("Failed to read CSV: {}", e);
            Status::internal("Failed to parse CSV")
        })?;

    let db_map: HashMap<_, _> = db_presets
        .iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    let csv_map: HashMap<_, _> = csv_presets
        .iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    let only_in_db: Vec<_> = db_map
        .iter()
        .filter(|(name, _)| !csv_map.contains_key(*name))
        .map(|(_, p)| (*p).clone())
        .collect();


    for preset in only_in_db {
        println!("Deprecated item: {} (Not removed!)", preset.name);
    }
    let only_in_csv: Vec<_> = csv_map
        .iter()
        .filter(|(name, _)| !db_map.contains_key(*name))
        .map(|(_, p)| (*p).clone())
        .collect();

    for preset in only_in_csv {
        sqlx::query!(
            r#"
            INSERT INTO preset_items (name, description, item_category_id, item_tier_id)
            VALUES (
                $1, $2,
                (SELECT id FROM item_categories WHERE name = $3),
                (SELECT id FROM item_tiers WHERE name = $4)
            )
            "#,
            preset.name,
            preset.description,
            preset.category,
            preset.tier,
        )
        .execute(&mut **tx)
        .await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;
        println!("New item: {} (inserted)", preset.name);
    }
    Ok(())
}