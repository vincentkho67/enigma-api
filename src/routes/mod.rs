use rocket_db_pools::Database;
use rocket::{Route, http::Status, response::status::Custom, serde::json::{json, Value}};
use std::error::Error;
pub mod user_routes;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];
    routes.extend(user_routes::routes());

    routes
}

pub fn server_error(e: Box<dyn Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!(e.to_string()))
}