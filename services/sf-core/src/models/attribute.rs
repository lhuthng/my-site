use sqlx::FromRow

#[derive(Debug, FromRow)]
pub struct Attribute {
    pub entity_id: i32,
    pub int_points: i32,
    pub str_points: i32,
    pub dex_points: i32,
    pub con_points: i32,
    pub lck_points: i32,
}