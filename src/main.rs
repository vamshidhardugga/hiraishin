use axum::{Json, Router, http::StatusCode, routing::get, serve};
use serde::Serialize;
use std::{borrow::Cow, env};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("PORT environment variable must be set");
    let app = Router::new()
        .route("/", get(index))
        .route("/status", get(status))
        .fallback(not_found);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("failed to bind to network port");
    serve(listener, app).await.expect("failed to start the axum server");
}

#[derive(Serialize)]
struct Index {
    message: Cow<'static, str>,
}

async fn index() -> (StatusCode, Json<Index>) {
    (
        StatusCode::OK,
        Json(Index {
            message: "Hiraishin - Redirection Engine".into(),
        }),
    )
}

async fn status() -> (StatusCode, Json<Index>) {
    (
        StatusCode::OK,
        Json(Index {
            message: "Server is up and running".into(),
        }),
    )
}

async fn not_found() -> (StatusCode, Json<Index>) {
    (
        StatusCode::NOT_FOUND,
        Json(Index {
            message: "Requested resource not found".into(),
        }),
    )
}
