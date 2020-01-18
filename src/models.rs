use super::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: Option<&'a String>,
    pub phone_number: Option<&'a String>,
}
