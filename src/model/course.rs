use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::{schema::courses};

#[derive(Queryable)]
pub struct Course {
    pub id: i32,
    pub course_type: String,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub duration_in_days: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=courses)]
pub struct NewCourse {
    pub name: String,
    pub course_type: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub duration_in_days: i32,
}
