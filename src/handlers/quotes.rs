use axum::{extract, http, routing::post, Router};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
struct CreateQuote {
    book: String,
    quote: String,
}

async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> http::StatusCode {
    println!("{:?}", payload);
    http::StatusCode::CREATED
}

pub fn api_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(create_quote))
        .with_state(pool)
}
