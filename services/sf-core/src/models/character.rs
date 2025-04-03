use crate::schema::characters;
use diesel::{Queryable, Insertable};

#[derive(Debug, Clone, Copy, SqlType, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum CharacterClass {
    warrior,
    mage,
    archer,
}

#[derive(Debug, Queryable)]
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