#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::user::User;

use crate::db::Connection;
use crate::schema::users;
use crate::schema::users::dsl::*;

pub fn get_user(user_id: i32, connection: &Connection) -> QueryResult<User> {
    users::table.find(user_id).get_result::<User>(connection)
}
