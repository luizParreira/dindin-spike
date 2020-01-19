use super::models::{
    AuthenticationPin, AuthenticationPinAttempt, NewAuthPinAttempt, NewPin, NewUser, User,
};
use super::schema::authentication_pins::dsl::{id, user_id};
use super::schema::users::dsl::{email, phone_number};
use super::schema::{authentication_pin_attempts, authentication_pins, users};
use chrono::Duration;
use chrono::Utc;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::RunQueryDsl;
use rocket_contrib::databases::database;

use diesel::PgConnection;
use diesel::QueryDsl;

#[database("diesel_postgres_pool")]
pub struct DBConn(PgConnection);

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

pub fn get_latest_auth_pin(conn: &PgConnection, user: &User) -> Option<AuthenticationPin> {
    authentication_pins::table
        .filter(user_id.eq(user.id))
        .order_by(id.desc())
        .first(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

fn create_auth_pin(conn: &PgConnection, user: &User, pin: String) -> AuthenticationPin {
    let new_pin = NewPin {
        user_id: user.id,
        pin,
        created_at: Utc::now(),
    };

    diesel::insert_into(authentication_pins::table)
        .values(&new_pin)
        .get_result(conn)
        .expect("Error saving new pin")
}

pub fn create_authentication_pin(
    conn: &PgConnection,
    user: &User,
    pin: String,
) -> AuthenticationPin {
    let latest_pin = get_latest_auth_pin(conn, user);
    let now = Utc::now();

    match latest_pin {
        Some(auth_pin) => {
            let minutes_ago = now - Duration::minutes(10);
            if auth_pin.created_at > minutes_ago {
                auth_pin
            } else {
                create_auth_pin(conn, user, pin)
            }
        }
        None => create_auth_pin(conn, user, pin),
    }
}

pub fn create_authentication_pin_attempt(
    conn: &PgConnection,
    user: &User,
    pin_input: String,
) -> AuthenticationPinAttempt {
    let new_pin_attempt = NewAuthPinAttempt {
        user_id: user.id,
        pin: pin_input,
        created_at: Utc::now(),
    };

    diesel::insert_into(authentication_pin_attempts::table)
        .values(&new_pin_attempt)
        .get_result(conn)
        .expect("Error saving pin auth attempt")
}
