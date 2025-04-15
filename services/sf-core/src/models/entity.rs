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
    pub int_points: i32,
    pub str_points: i32,
    pub dex_points: i32,
    pub con_points: i32,
    pub lck_points: i32,
}