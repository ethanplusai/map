use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

const INDEX_HTML: &str = include_str!("../static/index.html");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_index))
        .nest_service("/animations", ServeDir::new("static/animations"))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4180));
    println!("\n  ✨ Lottie Animation Gallery running at: http://{}\n", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    if let Ok(content) = std::fs::read_to_string("static/index.html") {
        Html(content)
    } else {
        Html(INDEX_HTML.to_string())
    }
}
