use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::roles;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name= roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct UpdateRole {
    pub code: Option<String>,
    pub name: Option<String>
}

