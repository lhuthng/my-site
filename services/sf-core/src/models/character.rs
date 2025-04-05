use crate::schema::characters;

use sqlx::FromRow;
use sqlx::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
#[sqlx(type_name = "item_type")]
pub enum CharacterClass {
    warrior,
    mage,
    archer,
}

#[derive(FromRow)]
pub struct Character {
    pub id: i32,
    pub user_id: i32,
    #[diesel(column_name = "class")] // Map the "class" column to the `character_class` field
    pub character_class: CharacterClass,
    pub entity_id: i32,
    pub name: String,
    pub level: i32,
    pub gold: i32,
    pub exp: i32,
    pub mushroom: i32,
}