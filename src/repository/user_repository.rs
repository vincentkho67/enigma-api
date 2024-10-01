use diesel::{ExpressionMethods, GroupedBy, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{middleware::authorization::hash_password, model::{pagination::{PaginatedResponse, PaginationParams}, role::{Role, RoleCode}, user::{NewUserAPI, NewUserCLI, UpdateUser, User}, users_roles::{NewUserRole, UserRole}}, schema::{roles, users, users_roles}};

use super::role_repository::{RoleRepository, UserRoleRepository};

pub struct UserRepository;

impl UserRepository {
    pub async fn index(
        c: &mut AsyncPgConnection,
        pagination: PaginationParams
    ) -> Result<PaginatedResponse<User>, diesel::result::Error> {
        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);
        let offset = (page - 1) * per_page;

        let users = users::table
            .offset(offset)
            .limit(per_page)
            .load::<User>(c)
            .await?;

        let total = users::table
            .count()
            .get_result::<i64>(c)
            .await?;

        Ok(PaginatedResponse::new(users, total, page, per_page))
    }

    pub async fn index_cli(c: &mut AsyncPgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(c).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c).await?
            .grouped_by(&users);
        
        Ok(users.into_iter().zip(result).collect())
    }

    pub async fn show(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }

    pub async fn show_by_email(c: &mut AsyncPgConnection, email: &String) -> QueryResult<User> {
        users::table.filter(users::email.eq(email)).get_result(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, mut user: NewUserAPI) -> QueryResult<User> {
        user.password = hash_password(user.password).unwrap();
        diesel::insert_into(users::table)
            .values(user)
            .get_result(c)
            .await
    }

    pub async fn create_with_cli(c: &mut AsyncPgConnection, mut user: NewUserCLI, role_codes: Vec<RoleCode>) -> QueryResult<User> {
        user.password = hash_password(user.password).unwrap();
        let user: User = diesel::insert_into(users::table)
            .values(user)
            .get_result(c)
            .await?;

        for role_code in role_codes {
            match RoleRepository::show_by_code(c, &role_code).await {
                Ok(role) => {
                    let new_user_role = NewUserRole { user_id: user.id, role_id: role.id };
                    UserRoleRepository::create(c, new_user_role).await?;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(user)
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, params: UpdateUser) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(params)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}
