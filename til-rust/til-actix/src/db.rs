use postgres::NoTls;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;

pub type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

pub fn get_db_pool() -> Pool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(db_url.parse().unwrap(), NoTls);
    r2d2::Pool::new(manager).expect("Failed to create DB Pool")
}
