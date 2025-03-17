mod routers;
mod config;
mod model;
mod handler;

use std::net::SocketAddr;
use config::connect_database;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = connect_database().await;
    println!("Connected to database");
    // Build our application with a route
    let app = routers::create_router(pool);

    // Specify the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("PORT").expect("PORT must be set").parse().unwrap()));
    println!("Listening on {}", addr);

    // Run the app with hyper
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
