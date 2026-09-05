use axum::{Router, routing::get, serve};
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let rust_log = env::var("RUST_LOG").expect("RUST_LOG environment variable must be set");
    tracing_subscriber::registry()
        .with(EnvFilter::new(&rust_log))
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
    let port = env::var("PORT").expect("PORT environment variable must be set");
    let address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind to network port");
    serve(listener, app).await.expect("failed to start the axum server");
}

async fn index() -> &'static str {
    "Hiraishin"
}
