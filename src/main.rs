use rocket_db_pools::Database;

mod routes;
mod repository;
mod model;
mod schema;
mod database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            routes::users::get_all,
            routes::users::get_one,
            routes::users::create,
            routes::users::update,
            routes::users::delete
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
