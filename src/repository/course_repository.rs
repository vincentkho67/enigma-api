use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::course::{NewCourse, UpdateCourse, Course}, schema::courses};

pub struct CourseRepository;

impl CourseRepository {
    pub async fn index(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Course>>{
        courses::table.limit(limit).get_results(c).await
    }
    pub async fn show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Course> {
        courses::table.find(id).get_result(c).await
    }
    pub async fn create(c: &mut AsyncPgConnection, mut course: NewCourse) -> QueryResult<Course> {
        let duration = course.end_date.signed_duration_since(course.start_date).num_days() as i32;
        course.duration_in_days = Some(duration);

        diesel::insert_into(courses::table)
            .values(course)
            .get_result(c)
            .await
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
