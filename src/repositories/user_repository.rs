#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::user::User;

//use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::users;
use crate::schema::users::dsl::*;

pub fn get_user(user_id: u64, conn: &MysqlConnection) -> QueryResult<User> {
    users::table.find(user_id).get_result::<User>(conn)
}
