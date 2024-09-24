use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::user::{NewUser, UpdateUser, User}, schema::users};

pub struct UserRepository;

impl UserRepository {
    pub async fn index(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>>{
        users::table.limit(limit).get_results(c).await
    }
    pub async fn show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }
    pub async fn create(c: &mut AsyncPgConnection, user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, params: UpdateUser) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(params)
            .get_result(c)
            .await
    }
    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}
