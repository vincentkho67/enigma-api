use diesel::{ExpressionMethods, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::{attendance::{Attendance, NewAttendance}, pagination::{PaginatedResponse, PaginationParams}, users_courses::{UpdateUserCourse, UserCourse}}, schema::attendance};
use diesel::QueryDsl;

use super::course_repository::UserCourseRepository;

pub struct AttendanceRepository;

impl AttendanceRepository {
    pub async fn index_by_user_course(
        c: &mut AsyncPgConnection,
        pagination: PaginationParams,
        user_course_id: i32
    ) -> Result<PaginatedResponse<Attendance>, diesel::result::Error> {
        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let attendances = attendance::table
            .offset(offset)
            .limit(per_page)
            .filter(attendance::user_course_id.eq(user_course_id))
            .load(c)
            .await?;

        let total = attendance::table
            .filter(attendance::user_course_id.eq(user_course_id))
            .count()
            .get_result::<i64>(c)
            .await?;

        Ok(PaginatedResponse::new(attendances, total, page, per_page))
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