use crate::schema::inventories;
use diesel::{Queryable, Insertable};

#[derive(Debug, Queryable)]
pub struct Inventory {
    pub id: i32,
    pub character_id: i32,
    pub capacity: i32,
}