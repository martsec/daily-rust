use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

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
    println!("--> HANDLER   hello");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

async fn handler_path_param(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}!</strong>"))
}
