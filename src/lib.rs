#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate diesel;

pub mod controllers;
pub mod db;
pub mod repositories;
pub mod schema;
pub mod models;
