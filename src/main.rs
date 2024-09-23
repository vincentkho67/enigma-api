use rocket_db_pools::Database;

mod routes;
mod repository;
mod model;
mod schema;
mod database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/api", routes::routes())
        .attach(routes::DbConn::init())
        .launch()
        .await;
}
