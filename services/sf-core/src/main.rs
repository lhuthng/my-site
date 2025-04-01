#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use std::env;

pub mod schema; 
pub mod models; 
mod db; 
fn main() {
    let connection = db::establish_connection();
}