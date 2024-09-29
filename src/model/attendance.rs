use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::attendance;

#[derive(Queryable)]
pub struct Attendance {
    pub _id: i32,
    pub _user_course_id: i32,
    pub _date: NaiveDateTime,
    pub _status: String,
    pub _notes: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=attendance)]
pub struct NewAttendance {
    pub user_course_id: i32,
    pub date: NaiveDateTime,
    pub status: String,
    pub notes: Option<String>,
}