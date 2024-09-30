use std::str::FromStr;

use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{model::{role::{NewRole, RoleCode}, user::NewUserCLI}, repository::{role_repository::RoleRepository, user_repository::UserRepository}};

async fn load_db_connection() -> AsyncPgConnection{
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot retrieve DB URL from environment.");

    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to DB.")
}

// User Section Commands
pub async fn create_user(email: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let new_user = NewUserCLI {
        email,
        password
    };
    let role_enums = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();
    let user = UserRepository::create_with_cli(&mut c, new_user, role_enums).await.unwrap();
    println!("Created user: {:?}", user);

    let roles = RoleRepository::show_by_user(&mut c, &user).await.unwrap();
    println!("Roles: {:?}", roles);
}

pub async fn index_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::index_cli(&mut c).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(_id: i32) {
    let mut _c = load_db_connection().await;

}

// Role Section Commands
pub async fn create_role(code: String, name: String) {
    let mut c = load_db_connection().await;

    let new_role = NewRole {
        code : RoleCode::from_str(&code).unwrap(),
        name
    };

    let role = RoleRepository::create(&mut c, new_role).await.unwrap();
    println!("Role created: {:?}", role);
}

pub async fn index_roles() {
    let mut _c = load_db_connection().await;

}

pub async fn delete_role(_id: i32) {
    let mut _c = load_db_connection().await;

}