pub mod config;
pub mod page;
pub mod template;

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = page::writing::add_routes(Router::new()).nest_service("/www", ServeDir::new("www"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
