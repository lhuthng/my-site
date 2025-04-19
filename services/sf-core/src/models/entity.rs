use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, EnumString)]
#[strum(serialize_all = "snake_case")]
#[sqlx(type_name = "entity_type", rename_all = "snake_case")]
pub enum EntityType {
    Character,
    Item
}

#[derive(Debug, FromRow)]
pub struct Entity {
    pub id: i32,
    pub kind: EntityType
}

#[derive(Debug, FromRow)]
pub struct Attribute {
    pub entity_id: i32,
    pub int: i32,
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub lck: i32,
}