#![allow(proc_macro_derive_resolution_fallback)]

//use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::sql_types::*;
use crate::schema::users;

//#[table_name = "users"]
#[derive(Queryable, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    pub authenticated_at: Option<chrono::naive::NaiveDateTime>,
    pub created_at: Option<chrono::naive::NaiveDateTime>,
    pub deleted_at: Option<chrono::naive::NaiveDateTime>,
}
