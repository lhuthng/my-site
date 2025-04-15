use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "container_type", rename_all = "snake_case")]
pub enum ContainerType {
    Inventory,
    Shop,
}

#[derive(Debug, FromRow)]
pub struct Container {
    pub id: i32,
    pub character_id: Uuid,
    pub kind: ContainerType,
    pub capacity: i32,
}