mod db;
mod insight;

use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = db::connect().await;
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/insight/ringkas", get(insight::ringkas))
        .route("/insight/harga", get(insight::harga))
        .route("/ingest/event", post(insight::terima_event))
        .with_state(pool);
    let l = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(l, app).await.unwrap();
}
