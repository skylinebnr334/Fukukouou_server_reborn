use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
pub type Pool=r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();
    let db_url=match std::env::var("DATABASE_URL"){
        Ok(url) => url,
        Err(_) => "sample.db".to_string(),
    };
    let manager=ConnectionManager::<SqliteConnection>::new(db_url);
    let pool:Pool=r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}