use crate::schema::items;
use diesel::{Queryable, Insertable};

#[derive(Debug, Clone, Copy, SqlType, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum ItemType {
    weapon,
    helment,
    robe,
    gloves,
    boots,
    necklace,
    ring,
    amulet,
    belt,
}

#[derive(Debug, Queryable)]
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