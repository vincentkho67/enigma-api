use diesel::{ExpressionMethods, QueryDsl, QueryResult, BelongingToDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{model::{role::{NewRole, Role}, user::User, users_roles::{NewUserRole, UserRole}}, schema::{roles, users_roles}};
pub struct RoleRepository;

impl RoleRepository {
    pub async fn _index(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Role>>{
        roles::table.limit(limit).get_results(c).await
    }

    pub async fn index_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }
    pub async fn _show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Role> {
        roles::table.find(id).get_result(c).await
    }

    pub async fn show_by_code(c: &mut AsyncPgConnection, role_code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(role_code)).first(c).await
    }
    
    pub async fn show_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results::<UserRole>(c).await?;

        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        
        Self::index_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(role)
            .get_result(c)
            .await
    }

    pub async fn _delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(c).await
    }
}

pub struct UserRoleRepository;

impl UserRoleRepository {
    pub async fn create(c: &mut AsyncPgConnection, params: NewUserRole) -> QueryResult<UserRole> {
        diesel::insert_into(users_roles::table)
                        .values(params)
                        .get_result(c)
                        .await
    }
}