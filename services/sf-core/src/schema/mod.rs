use diesel::prelude::*;
use diesel::pg::PgConnection;

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar, 
    }
}