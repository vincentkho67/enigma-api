use crate::{database::db::CacheConn, middleware::authorization::authorize_user, model::user::Credential, repository::user_repository::UserRepository};
use rocket::{response::status::Custom, serde::json::{json, Json, Value}, Route, routes};
use crate::routes::{DbConn, server_error};
use rocket_db_pools::Connection;

pub fn routes() -> Vec<Route> {
    routes![login]
}

#[rocket::post("/auth/login", format="json", data="<credential>")]
pub async fn login(mut c: Connection<DbConn>, mut cache: Connection<CacheConn>, credential: Json<Credential>) -> Result<Value, Custom<Value>> {
    UserRepository::show_by_email(&mut c, &credential.email).await
        .map(|u| {
            if let Ok(token) = authorize_user(&u, credential.into_inner()){
                return json!({
                    "token": token
                })
            }
            json!("Invalid Credential")
        })
        .map_err(|e| server_error(e.into()))
}
