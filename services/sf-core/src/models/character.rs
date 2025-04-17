use sqlx::{Type, FromRow};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, EnumString)]
#[strum(serialize_all = "PascalCase")]
#[sqlx(type_name = "character_class", rename_all = "snake_case")]
pub enum CharacterClass {
    Warrior,
    Mage,
    Archer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, EnumString)]
#[strum(serialize_all = "PascalCase")]
#[sqlx(type_name = "resource_type", rename_all = "snake_case")]
pub enum ResourceType {
    Gold,
    Mushroom,
}

#[derive(Debug, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub user_id: i32,
    pub entity_id: i32,
    pub class: CharacterClass,
    pub name: String,
    pub level: i32,
    pub exp: i32,
}

#[derive(Debug, FromRow)]
pub struct Resource {
    pub character_id: i32,
    pub kind: ResourceType,
    pub amount: i32,
}