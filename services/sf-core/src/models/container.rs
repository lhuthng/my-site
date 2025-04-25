use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, EnumString)]
#[strum(serialize_all = "PascalCase")]
#[sqlx(type_name = "container_type", rename_all = "snake_case")]
pub enum ContainerType {
    Inventory,
    GearShop,
    MagicShop
}

#[derive(Debug, FromRow)]
pub struct Container {
    pub id: i32,
    pub character_id: Uuid,
    pub kind: ContainerType,
    pub capacity: i32,
}

#[derive(Debug, FromRow)]
pub struct ResSlot {
    pub index: i32,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub item_category_id: i32,
    pub item_sub_category_id: i32,
    pub item_tier_id: i32,
}

#[derive(Debug, FromRow)]
pub struct ResEquipment {
    pub int: i32,
    pub str: i32,
    pub dex: i32,
    pub lck: i32,
    pub con: i32,
}

#[derive(Debug, FromRow)]
pub struct ResAccessory {
}

#[derive(Debug, FromRow)]
pub struct ResArmor {
    pub armor_points: i32,
}

#[derive(Debug, FromRow)]
pub struct ResWeapon {
    pub min_dmg: i32,
    pub max_dmg: i32,
}

#[derive(Debug, FromRow)]
pub struct ResShield {
    pub block_points: i32,
}

#[derive(Debug, FromRow)]
pub struct ResPotion {
    pub attr: String,
    pub value: i32,
}
