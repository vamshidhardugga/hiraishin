use axum::{Router, routing::get, serve};
use std::env;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("PORT environment variable must be set");
    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
                .flatten_event(true),
        )
        .init();
    let app = Router::new().route("/", get(index));
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("failed to bind to network port");
    serve(listener, app).await.expect("failed to start the axum server");
}

async fn index() -> &'static str {
    info!("Hiraishin - Redirection Engine");
    "Hiraishin - Redirection Engine"
}
