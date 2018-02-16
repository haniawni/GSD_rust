use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

// An alias to the type for a pool of Diesel PG connections.
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool init error")
}