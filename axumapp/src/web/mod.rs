use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

pub mod routes_login;
pub mod routes_tickets;

pub const AUTH_TOKEN: &str = "auth-token";

// Hello routes
pub fn hello_routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_path_param))
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    pub(crate) name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - hello_query", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

async fn handler_path_param(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - hello_path", "HANDLER");
    Html(format!("Hello <strong>{name}!</strong>"))
}
