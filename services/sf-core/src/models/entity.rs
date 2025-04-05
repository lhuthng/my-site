use craete::schema::entities;

use sqlx::FromRow;
use sqlx::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
#[sqlx(type_name = "item_type")]
pub enum EntityType {
    item,
    character
}

#[derive(FromRow)]
pub struct Entity {
    pub id: i32,
    #[diesel(column_name = "type")]
    pub entity_type: EntityType,
}