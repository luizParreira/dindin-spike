use crate::db;
use chrono::Duration;
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub enum PinAuthError {
    InexistentUser,
    InexistentPin,
    MismatchedPins,
    ExpiredPin,
}

#[derive(Serialize, Deserialize)]
pub struct LoginClaims {
    iat: DateTime<Utc>,
    pub sub: i32,
}

pub fn authenticate_pin(
    conn: &PgConnection,
    email: &Option<String>,
    phone_number: &Option<String>,
    pin: String,
) -> Result<LoginClaims, PinAuthError> {
    let now = chrono::Utc::now();

    let user = match db::find_user_by(conn, email, phone_number) {
        None => return Err(PinAuthError::InexistentUser),
        Some(user) => user,
    };

    let latest_pin = match db::get_latest_auth_pin(conn, &user) {
        None => return Err(PinAuthError::InexistentPin),
        Some(matched_pin) => matched_pin,
    };

    let ten_minutes_ago = now - Duration::minutes(10);
    if &latest_pin.created_at < &ten_minutes_ago {
        return Err(PinAuthError::ExpiredPin);
    }

    let attempt = db::create_authentication_pin_attempt(conn, &user, pin);

    if attempt.pin == latest_pin.pin {
        Ok(LoginClaims {
            iat: now,
            sub: user.id,
        })
    } else {
        Err(PinAuthError::MismatchedPins)
    }
}
