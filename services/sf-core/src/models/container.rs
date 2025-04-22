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