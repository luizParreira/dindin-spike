#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod authenticate;
pub mod db;
pub mod models;
pub mod schema;
pub mod server;
