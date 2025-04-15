use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};
use crate::models::character::CharacterClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "item_type", rename_all = "snake_case")]
pub enum ItemType {
    Helmet,
    Chest,
    Gloves,
    Boots,
    Necklace,
    Belt,
    Ring,
    Amulet,
    Shield,
    Weapon,
    Potion,
    SubWeapon,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "item_tier", rename_all = "snake_case")]
pub enum ItemTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub kind: ItemType,
    pub tier: ItemTier,
    pub description: String,
    pub price: i32,
}

#[derive(Debug, FromRow)]
pub struct EquippableItem {
    pub item_id: i32,
    pub entity_id: i32,
    pub job: CharacterClass,
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