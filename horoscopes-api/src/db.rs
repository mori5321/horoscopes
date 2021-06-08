// use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

// pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
// fn mysql_pool(db_url: &str) -> MysqlPool {
//     let manager = ConnectionManager::<MysqlConnection>::new(db_url);
//     Pool::new(manager).expect("Mysql pool could not be created.")
// }
//
// pub fn gen_mysql_pool() -> MysqlPool {
//     let database_url =
//         env::var("DATABASE_URL").expect("DATABASE_URL is invalid.");
//     mysql_pool(database_url.as_str())
// }
//

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
fn pg_pool(db_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres pool could not be created.")
}

pub fn gen_db_pool() -> DbPool {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is invalid.");
    pg_pool(database_url.as_str())
}
