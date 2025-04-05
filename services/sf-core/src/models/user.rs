use crate::schema::users;

use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub external_id: String,
    pub last_update: DateTime<Utc>,
    pub username: String,
}