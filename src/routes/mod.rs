use rocket::{routes, fairing::{Fairing, Info, Kind}, http::Status, request::{FromRequest, Outcome}, response::status::Custom, serde::json::{json, Value}, Request, Response, Route};
use std::error::Error;
use crate::{database::db::{CacheConn, DbConn}, model::{role::RoleCode, user::User}, repository::{role_repository::RoleRepository, user_repository::UserRepository}};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

pub mod authorization_route;
pub mod user_route;
pub mod course_route;
pub mod attendance_route;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];
    routes.extend(routes![options]);
    routes.extend(authorization_route::routes());
    routes.extend(user_route::routes());
    routes.extend(course_route::routes());
    routes.extend(attendance_route::routes());

    routes
}

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Just to add CORS header via the fairing.
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = req.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("Can not connect to redis in request guard");
            let mut db = req.guard::<Connection<DbConn>>().await
                .expect("Can not connect to postgres in request guard");
            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::show(&mut db, user_id).await {
                    return Outcome::Success(user);
                }                    
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub struct AdminUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<User>().await {
            Outcome::Success(user) => {
                let mut db = req.guard::<Connection<DbConn>>().await
                    .expect("Can not connect to postgres in request guard");
                if let Ok(roles) = RoleRepository::show_by_user(&mut db, &user).await {
                    let is_admin = roles.iter().any(|v| match v.code {
                        RoleCode::Admin => true,
                        RoleCode::Trainer => true,
                        _ => false
                    });
                    if is_admin {
                        return Outcome::Success(AdminUser(user));
                    }
                }
                Outcome::Error((Status::Unauthorized, ()))
            },
            _ => Outcome::Error((Status::Unauthorized, ()))
        }
    }
}


pub fn server_error(e: Box<dyn Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!(e.to_string()))
}