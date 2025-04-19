use sqlx::FromRow;


#[derive(Debug, FromRow)]
pub struct PresetItem {
    pub id: i16,
    pub name: String,
    pub description: String,
    pub item_category_id: i32,
    pub item_tier_id: i32,
}

#[derive(Debug, FromRow)]
pub struct Item {
    pub id: i32,
    pub preset_item_id: i16,
    pub overridden_item_tier_id: Option<i16>,
    pub price: i32,
}

#[derive(Debug, FromRow)]
pub struct EquippableItem {
    pub item_id: i32,
    pub entity_id: i32,
    pub job_id: i16,
}

#[derive(Debug, FromRow)]
pub struct Potion {
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Debug, FromRow)]
pub struct ArmorItem {
    pub item_id: i32,
    pub armor_points: i32,
}

#[derive(Debug, FromRow)]
pub struct WeaponItem {
    pub item_id: i32,
    pub min_damage: i32,
    pub max_damage: i32,
}

#[derive(Debug, FromRow)]
pub struct ShieldItem {
    pub item_id: i32,
    pub armor_points: i32,
    pub block_points: i32,
}

#[derive(Debug, FromRow)]
pub struct AccessoryItem {
    pub item_id: i32,
}