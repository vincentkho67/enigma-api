use rocket_db_pools::Database;
use rocket::Route;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub mod user_routes;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];
    routes.extend(user_routes::routes());

    routes
}