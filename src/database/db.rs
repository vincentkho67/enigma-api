use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(pub rocket_db_pools::diesel::PgPool);