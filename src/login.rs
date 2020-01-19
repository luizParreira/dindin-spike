use crate::db;
use crate::models::User;
use crate::notifications;
use rand::{thread_rng, Rng};

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

fn generate_pin() -> String {
    let mut rng = thread_rng();
    let mut pin = String::from("");

    for _ in 0..4 {
        pin = format!("{}{}", pin, rng.gen_range(0, 10));
    }

    pin
}

#[rocket::post("/login", data = "<data>")]
pub fn login(conn: db::DBConn, data: Json<Request>) -> Json<User> {
    let (email, phone): (Option<String>, Option<String>) = match &data.auth_input {
        AuthInput::Email { email } => (Some(email.to_string()), None),
        AuthInput::PhoneNumber { phone } => (None, Some(phone.to_string())),
    };

    let user = match db::find_user_by(&conn, &email, &phone) {
        Some(user) => user,
        None => db::create_user(&conn, email, phone),
    };

    let pin = generate_pin();
    let pin = db::create_authentication_pin(&conn, &user, pin);

    match &user.email {
        Some(_) => notifications::send_email(&user, &pin),
        None => notifications::send_text_message(&user, &pin),
    }

    Json(user)
}
