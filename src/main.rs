use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::health::health_check))
        .nest("/quotes", handlers::quotes::api_routes(pool));

    println!("Server listening on: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
