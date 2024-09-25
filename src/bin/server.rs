extern crate enigma_api;
use rocket_db_pools::Database;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/api", enigma_api::routes::routes())
        .attach(enigma_api::routes::DbConn::init())
        .launch()
        .await;
}
