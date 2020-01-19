use super::models::{NewUser, User};
use super::schema::users;
use super::schema::users::dsl::*;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::RunQueryDsl;
use rocket_contrib::databases::database;

use diesel::PgConnection;
use diesel::QueryDsl;

#[database("diesel_postgres_pool")]
pub struct LogsDbConn(PgConnection);

pub fn create_user(
    conn: &PgConnection,
    email_input: Option<String>,
    phone_number_input: Option<String>,
) -> User {
    let new_user = NewUser {
        email: email_input,
        phone_number: phone_number_input,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn find_user_by(
    conn: &PgConnection,
    email_input: &Option<String>,
    phone_number_input: &Option<String>,
) -> Option<User> {
    users::table
        .filter(email.eq(email_input))
        .or_filter(phone_number.eq(phone_number_input))
        .first(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}
