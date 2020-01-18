#![feature(proc_macro_hygiene, decl_macro)]
// use crate::models::{NewPost, Post};

#[macro_use]
extern crate diesel;

use rocket::{routes, Rocket};
use rocket_contrib::databases::database;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type", content = "attributes")]
enum AuthInput {
    Email { email: String },
    PhoneNumber { phone: String },
}

#[derive(Deserialize)]
struct Request {
    auth_input: AuthInput,
}

#[rocket::post("/login")]
fn authenticate() -> Json<bool> {
    Json(true)
}

#[database("postgresql_logs")]
struct LogsDbConn(diesel::PgConnection);

pub fn init() -> Rocket {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .mount("/", routes![authenticate])
}
fn main() {
    init().launch();
}
