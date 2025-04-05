use crate::schema::items;

use sqlx::FromRow;
use sqlx::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
#[sqlx(type_name = "item_type")]
pub enum ItemType {
    weapon,
    helmet,
    robe,
    gloves,
    boots,
    necklace,
    ring,
    amulet,
    belt,
}

#[derive(FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: String,
    pub inventory_id: i32,
    pub entity_id: i32,
    pub slot_number: i32,
    pub armor_points: i32,
}