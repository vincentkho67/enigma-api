use rocket_db_pools::Connection;
use rocket::{http::Status, response::status::{Custom, NoContent}, routes, serde::json::{json, Json, Value}, Route};
use crate::{model::user::{NewUser, UpdateUser}, repository::user_repository::UserRepository};
use crate::routes::{DbConn, server_error};

pub fn routes() -> Vec<Route> {
    routes![get_all, get_one, create, update, delete]
}

#[rocket::get("/users")]
pub async fn get_all(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    UserRepository::index(&mut db, 100).await
        .map(|u| json!(u))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/users/<id>")]
pub async fn get_one(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    UserRepository::show(&mut db, id).await
        .map(|u| json!(u))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/users", format="json", data="<params>")]
pub async fn create(mut db: Connection<DbConn>, params: Json<NewUser>) -> Result<Custom<Value>, Custom<Value>>{
    UserRepository::create(&mut db, params.into_inner()).await
        .map(|u| Custom(Status::Created, json!(u)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/users/<id>", format="json", data="<params>")]
pub async fn update(mut db: Connection<DbConn>, id: i32, params: Json<UpdateUser>) -> Result<Custom<Value>, Custom<Value>>{
    UserRepository::update(&mut db, id, params.into_inner()).await
        .map(|u| Custom(Status::Ok, json!(u)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/users/<id>",)]
pub async fn delete(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
    UserRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}