use super::schema::users;
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
