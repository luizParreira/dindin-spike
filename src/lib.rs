#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod authenticate;
pub mod db;
pub mod login;
pub mod models;
pub mod notifications;
pub mod schema;
pub mod server;
