use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub phone_number: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_id: Option<String>,
    pub profile_picture_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=users)]
pub struct NewUserCLI {
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=users)]
pub struct NewUserAPI {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(Deserialize)]
pub struct Credential {
    pub email: String,
    pub password: String,
}