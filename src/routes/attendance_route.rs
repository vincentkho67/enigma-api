use rocket_db_pools::Connection;
use rocket::{http::Status, response::status::Custom, routes, serde::json::{json, Json, Value}, Route};
use crate::{model::{attendance::NewAttendance, pagination::PaginationParams}, repository::attendance_repository::AttendanceRepository};
use crate::routes::{DbConn, server_error};

use super::AdminUser;

pub fn routes() -> Vec<Route> {
    routes![get_all_by_user_course, create]
}

#[rocket::get("/attendance/<id>?<page>&<per_page>")]
pub async fn get_all_by_user_course(
    mut db: Connection<DbConn>, 
    id: i32,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<Json<Value>, Custom<Value>>{
    let pagination = PaginationParams { page, per_page };

    AttendanceRepository::index_by_user_course(&mut *db, pagination, id).await
        .map(|paginated_response| Json(json!(paginated_response)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/attendance", format="json", data="<params>")]
pub async fn create(mut db: Connection<DbConn>, params: Json<NewAttendance>, _user: AdminUser) -> Result<Custom<Value>, Custom<Value>>{
    AttendanceRepository::create(&mut db, params.into_inner()).await
        .map(|u| Custom(Status::Created, json!(u)))
        .map_err(|e| server_error(e.into()))
}