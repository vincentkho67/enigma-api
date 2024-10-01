use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::{course::{Course, NewCourse, UpdateCourse}, pagination::{PaginatedResponse, PaginationParams}, user::UserIds, users_courses::{NewUserCourse, UpdateUserCourse, UserCourse}}, schema::{courses, users_courses}};
use chrono::prelude::*;
use diesel::ExpressionMethods;


pub struct CourseRepository;

impl CourseRepository {
    pub async fn index(
        c: &mut AsyncPgConnection,
        pagination: PaginationParams
    ) -> Result<PaginatedResponse<Course>, diesel::result::Error> {
        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let courses = courses::table
            .offset(offset)
            .limit(per_page)
            .load::<Course>(c)
            .await?;

        let total = courses::table
            .count()
            .get_result::<i64>(c)
            .await?;

        Ok(PaginatedResponse::new(courses, total, page, per_page))
    }
    pub async fn show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Course> {
        courses::table.find(id).get_result(c).await
    }
    pub async fn create(c: &mut AsyncPgConnection, mut course: NewCourse) -> QueryResult<Course> {
        let mut current_date = course.start_date;
        let mut weekdays = 0;

        while current_date <= course.end_date {
            if current_date.weekday() != Weekday::Sat && current_date.weekday() != Weekday::Sun {
                weekdays += 1;
            }
            current_date = current_date + chrono::Duration::days(1);
        }

        course.duration_in_days = Some(weekdays);

        diesel::insert_into(courses::table)
            .values(course)
            .get_result(c)
            .await
    }
    pub async fn assign_student(c: &mut AsyncPgConnection, course_id: i32, user_ids: UserIds) -> Result<String, Box<dyn std::error::Error>> {
        for user_id in user_ids.user_ids {
            UserCourseRepository::create(
                c,
                NewUserCourse {
                    user_id,
                    course_id,
                    total_attendance: 0
                }
            ).await?;
        }
        Ok(String::from("Success!"))
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, params: UpdateCourse) -> QueryResult<Course> {
        diesel::update(courses::table.find(id))
            .set(params)
            .get_result(c)
            .await
    }
    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(courses::table.find(id)).execute(c).await
    }
}

pub struct UserCourseRepository;

impl UserCourseRepository {
    pub async fn show_by_user(c: &mut AsyncPgConnection, user_id: i32) -> QueryResult<UserCourse> {
        users_courses::table.filter(users_courses::user_id.eq(user_id)).first(c).await
    }

    pub async fn show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<UserCourse> {
        users_courses::table.find(id).get_result(c).await
    }
    pub async fn create(c: &mut AsyncPgConnection, params: NewUserCourse) -> QueryResult<UserCourse> {
        diesel::insert_into(users_courses::table)
            .values(params)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, params: UpdateUserCourse) -> QueryResult<UserCourse> {
        diesel::update(users_courses::table.find(id))
            .set(params)
            .get_result(c)
            .await
    }
}