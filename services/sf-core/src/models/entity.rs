use craete::schema::entities;
use diesel::{Queryable, Insertable};

#[derive(Debug, Clone, Copy, SqlType, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum EntityType {
    item,
    character
}

#[derive(Debug, Queryable)]
pub struct Entity {
    pub id: i32,
    #[diesel(column_name = "type")]
    pub entity_type: EntityType,
}