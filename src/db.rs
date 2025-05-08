use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
pub type Pool=r2d2