use diesel::{ExpressionMethods, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::{attendance::{Attendance, NewAttendance}, users_courses::{UpdateUserCourse, UserCourse}}, schema::attendance};
use diesel::QueryDsl;

use super::course_repository::UserCourseRepository;

pub struct AttendanceRepository;

impl AttendanceRepository {
    pub async fn index_by_user_course(c: &mut AsyncPgConnection, user_course_id: i32) -> QueryResult<Vec<Attendance>> {
        attendance::table.filter(attendance::user_course_id.eq(user_course_id)).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_attendance: NewAttendance) -> QueryResult<UserCourse> {
        diesel::insert_into(attendance::table)
            .values(&new_attendance)
            .execute(c)
            .await?;

        let user_course = UserCourseRepository::show(c, new_attendance.user_course_id).await?;

        UserCourseRepository::update(
            c, 
            user_course.id, 
            UpdateUserCourse {
                total_attendance: user_course.total_attendance + 1
            }
        ).await
    }
}