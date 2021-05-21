use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;


pub fn create_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("ERROR NO DB URL");
    let conn_manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
    .build(conn_manager)
    .expect("FAILED TO CREATE POOL")
}
