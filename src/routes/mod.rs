use rocket::{Route, http::Status, response::status::Custom, serde::json::{json, Value}};
use std::error::Error;
use crate::database::db::DbConn;

pub mod authorization_route;
pub mod user_route;
pub mod course_route;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];
    routes.extend(authorization_route::routes());
    routes.extend(user_route::routes());
    routes.extend(course_route::routes());

    routes
}

pub fn server_error(e: Box<dyn Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!(e.to_string()))
}