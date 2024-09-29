use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(pub rocket_db_pools::diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(pub rocket_db_pools::deadpool_redis::Pool);