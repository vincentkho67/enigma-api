use diesel::prelude::*;
use crate::schema::users_courses;

#[derive(Queryable)]
pub struct UserCourse {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub total_attendance: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_courses)]
pub struct NewUserCourse {
    pub user_id: i32,
    pub course_id: i32,
    pub total_attendance: i32,
}