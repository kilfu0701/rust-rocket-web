#![allow(proc_macro_derive_resolution_fallback)]

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::users;


#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}
