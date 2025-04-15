use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub external_id: String,
    pub username: String,
    pub last_update: NaiveDateTime,
}
