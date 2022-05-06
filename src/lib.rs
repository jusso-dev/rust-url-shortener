use dotenv::dotenv;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
.expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}