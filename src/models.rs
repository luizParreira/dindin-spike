use super::schema::{authentication_pin_attempts, authentication_pins, users};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct AuthenticationPin {
    pub id: i32,
    pub user_id: i32,
    pub pin: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "authentication_pins"]
pub struct NewPin {
    pub user_id: i32,
    pub pin: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct AuthenticationPinAttempt {
    pub id: i32,
    pub user_id: i32,
    pub pin: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "authentication_pin_attempts"]
pub struct NewAuthPinAttempt {
    pub user_id: i32,
    pub pin: String,
    pub created_at: DateTime<Utc>,
}
