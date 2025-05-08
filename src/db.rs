use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
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
}pub fn establish_connection_for_test() -> Pool {
    dotenv().ok();
    let db_url=":memory:";
    let manager=ConnectionManager::<SqliteConnection>::new(db_url);
    let pool:Pool=r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
pub fn test_transaction<F,R>(test_fn:F) -> Result<R,Error>
    where
        F:FnOnce(&SqliteConnection)->Result<R,Error>,
    {
        dotenv().ok();
        let db_url = "test.db";
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let mut connection = pool.clone().get().unwrap();
        Ok(connection.test_transaction::<_,Error,_>(|connection|
            test_fn(&connection)))
    }