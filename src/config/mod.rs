
use sqlx::{Pool, Postgres};
use std::env;
use dotenv::dotenv;

pub async fn connect_database() -> Pool<Postgres> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool");
    return pool
}