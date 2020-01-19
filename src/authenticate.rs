use crate::auth::{authenticate_pin, PinAuthError};
use crate::db;
use jsonwebtoken as jwt;
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct AuthParams {
    pin: String,
    phone_number: Option<String>,
    email: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    token: String,
    user_id: i32,
}

#[rocket::post("/authenticate", data = "<data>")]
pub fn authenticate(
    conn: db::DBConn,
    data: Json<AuthParams>,
) -> Result<Json<Response>, status::BadRequest<Json<PinAuthError>>> {
    match authenticate_pin(&conn, &data.email, &data.phone_number, data.pin.clone()) {
        Err(err) => Err(status::BadRequest(Some(Json(err)))),
        Ok(claims) => {
            let token = jwt::encode(
                &jwt::Header::default(),
                &claims,
                env::var("JWT_SECRET_KEY")
                    .expect("no JWT_SECRET_KEY no auth")
                    .as_ref(),
            )
            .unwrap();
            let response = Response {
                user_id: claims.sub,
                token,
            };
            Ok(Json(response))
        }
    }
}
