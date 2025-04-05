use crate::schema::inventories;

use sqlx::FromRow;

#[derive(FromRow)]
pub struct Inventory {
    pub id: i32,
    pub character_id: i32,
    pub capacity: i32,
}