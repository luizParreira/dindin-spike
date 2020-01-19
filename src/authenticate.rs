use crate::db;
use crate::models::User;

use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "attributes")]
pub enum AuthInput {
    Email { email: String },
    PhoneNumber { phone: String },
}

#[derive(Deserialize)]
pub struct Request {
    pub auth_input: AuthInput,
}

#[rocket::post("/authenticate", data = "<data>")]
pub fn authenticate(conn: db::LogsDbConn, data: Json<Request>) -> Json<User> {
    let (email, phone): (Option<String>, Option<String>) = match &data.auth_input {
        AuthInput::Email { email } => (Some(email.to_string()), None),
        AuthInput::PhoneNumber { phone } => (None, Some(phone.to_string())),
    };

    let user = match db::find_user_by(&conn, &email, &phone) {
        Some(user) => user,
        None => db::create_user(&conn, email, phone),
    };

    Json(user)
}
