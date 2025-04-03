use crate::schema::users;

use diesel::{Queryable, Insertable};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub external_id: String,
    pub last_update: DateTime<Utc>,
    pub username: String,
}

// #[derive(Insertable)]
// #[table_name = "users"]
// pub struct NewUser {
//     pub name: String,
//     pub email: String,
// }