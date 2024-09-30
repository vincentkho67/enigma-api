use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users_courses;
use crate::model::{course::Course, user::User};

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Course))]
#[diesel(table_name = users_courses)]
pub struct UserCourse {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub total_attendance: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users_courses)]
pub struct NewUserCourse {
    pub user_id: i32,
    pub course_id: i32,
    pub total_attendance: i32,
}
