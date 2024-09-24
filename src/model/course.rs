use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::courses;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub course_type: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub duration_in_days: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub name: String,
    pub course_type: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub duration_in_days: Option<i32>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = courses)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub course_type: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub duration_in_days: Option<i32>,
}