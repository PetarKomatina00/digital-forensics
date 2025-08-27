use axum::Router;
use tower_http::cors::{CorsLayer};

mod models;
mod repository;
mod endpoints;
mod utility;
mod constants;
use axum;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive(); // allows everything (for dev)
    let app: Router = Router::new().merge(repository::router()).layer(cors);

    let listener = tokio::net::TcpListener::bind(constants::AXUM_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
