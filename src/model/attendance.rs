use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::attendance;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Attendance {
    pub id: i32,
    pub user_course_id: i32,
    pub date: NaiveDateTime,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=attendance)]
pub struct NewAttendance {
    pub user_course_id: i32,
    pub date: NaiveDateTime,
    pub status: String,
    pub notes: Option<String>,
}