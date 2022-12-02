use std::{env};
use diesel::{
    r2d2::{ConnectionManager, Pool as TPool},
    PgConnection
};

use dotenvy::dotenv;

pub fn get_pool() -> TPool<ConnectionManager<PgConnection>>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager:ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    TPool::builder().build(manager).expect("Something went wrong trying to get a connection pool")
}

