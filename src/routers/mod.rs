use axum::{
    routing::{post, get},
    Router,
};

use sqlx::{Pool, Postgres};
use crate::handler::{get_all_stores, create_store, get_store_by_id, update_store_by_id, delete_store_by_id};

type DbPool = Pool<Postgres>;

// Function to create the router
pub fn create_router(pool : DbPool) -> Router {
    Router::new()
        .route("/stores", post(create_store).get(get_all_stores))
        .route("/stores/:id", get(get_store_by_id).put(update_store_by_id).delete(delete_store_by_id))
        .with_state(pool)
}