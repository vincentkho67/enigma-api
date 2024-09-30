use rocket_db_pools::Connection;
use rocket::{http::Status, response::status::{Custom, NoContent}, routes, serde::json::{json, Json, Value}, Route};
use crate::{model::{course::{NewCourse, UpdateCourse}, user::User}, repository::course_repository::CourseRepository};
use crate::routes::{DbConn, server_error};

use super::AdminUser;

pub fn routes() -> Vec<Route> {
    routes![get_all, get_one, create, update, delete]
}

#[rocket::get("/courses")]
pub async fn get_all(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    CourseRepository::index(&mut db, 100).await
        .map(|u| json!(u))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/courses/<id>")]
pub async fn get_one(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    CourseRepository::show(&mut db, id).await
        .map(|u| json!(u))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/courses", format="json", data="<params>")]
pub async fn create(mut db: Connection<DbConn>, params: Json<NewCourse>, _user: AdminUser) -> Result<Custom<Value>, Custom<Value>>{
    CourseRepository::create(&mut db, params.into_inner()).await
        .map(|u| Custom(Status::Created, json!(u)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/courses/<id>", format="json", data="<params>")]
pub async fn update(mut db: Connection<DbConn>, id: i32, params: Json<UpdateCourse>, _user: AdminUser) -> Result<Custom<Value>, Custom<Value>>{
    CourseRepository::update(&mut db, id, params.into_inner()).await
        .map(|u| Custom(Status::Ok, json!(u)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/courses/<id>",)]
pub async fn delete(mut db: Connection<DbConn>, id: i32, _user: AdminUser) -> Result<NoContent, Custom<Value>>{
    CourseRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}