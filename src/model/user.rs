use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_id: Option<String>,
    pub profile_picture_url: Option<String>,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


impl User {
    pub fn is_valid_role(role: &str) -> bool {
        matches!(role, "STUDENT" | "ADMIN" | "TRAINER")
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
}
