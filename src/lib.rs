#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use self::models::{NewUser, User};
use self::schema::users;
use diesel::PgConnection;

pub mod models;
pub mod schema;

pub fn create_user<'a>(
    conn: &PgConnection,
    phone_number: Option<&'a String>,
    email: Option<&'a String>,
) -> User {
    let new_user = NewUser {
        email,
        phone_number,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_results(conn)
        .expect("Error saving new user")
}
