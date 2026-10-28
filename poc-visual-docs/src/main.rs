use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::Path;
use tower_http::cors::CorsLayer;

const INDEX_HTML: &str = include_str!("../static/index.html");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/architecture.json", get(serve_architecture))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4173));
    println!("\n  🗺️  System Cartography Viewer running at: http://{}\n", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    // If static/index.html exists locally on disk, serve that for hot iteration, otherwise fallback to embedded
    if let Ok(content) = std::fs::read_to_string("static/index.html") {
        Html(content)
    } else {
        Html(INDEX_HTML.to_string())
    }
}

async fn serve_architecture() -> impl IntoResponse {
    // Read from disk dynamically if available so git hooks/updates reflect immediately without recompile
    let content = if Path::new("architecture.json").exists() {
        std::fs::read_to_string("architecture.json").unwrap_or_else(|_| include_str!("../architecture.json").to_string())
    } else {
        include_str!("../architecture.json").to_string()
    };

    (
        [("content-type", "application/json")],
        content,
    )
}

