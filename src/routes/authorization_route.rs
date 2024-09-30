use crate::{database::db::CacheConn, middleware::authorization::authorize_user, model::user::{Credential, User}, repository::user_repository::UserRepository};
use rocket::{http::Status, response::status::Custom, routes, serde::json::{json, Json, Value}, Route};
use crate::routes::{DbConn, server_error};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

pub fn routes() -> Vec<Route> {
    routes![login, me]
}

#[rocket::post("/auth/login", format="json", data="<credential>")]
pub async fn login(mut c: Connection<DbConn>, mut cache: Connection<CacheConn>, credential: Json<Credential>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::show_by_email(&mut c, &credential.email).await
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => Custom(Status::Unauthorized, json!("Invalid Credential!")),
                _ => server_error(e.into()),
            }
        })?;

    let session_id = authorize_user(&user, credential.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Invalid Credential!")))?;

    cache.set_ex::<String, i32, ()>(
        format!("sessions/{}", session_id), 
        user.id, 
        3*60*60
    )
    .await
    .map_err(|e| server_error(e.into()))?;
    
    Ok(json!(
        {
            "token": session_id
        }
    ))
}

#[rocket::get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}