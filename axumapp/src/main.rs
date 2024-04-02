#![allow(unused)]
use std::net::SocketAddr;

use axum::response::{IntoResponse, Response};
use axum::routing::get_service;
use axum::{self, middleware};
use axum::{response::Html, Router};
use serde::Deserialize;
use tokio;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

pub use crate::error::{Error, Result};

use crate::model::ModelController;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;

    // Require login
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_hello = Router::new()
        .merge(web::hello_routes())
        .merge(web::routes_login::routes())
        // Nice usage for prefixes
        .nest("/api", routes_apis)
        //IMPORTANT Layers get executed from bottom to top
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        // Because we would have a conflict
        .fallback_service(routes_static());

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> LISTENING on {addr}\n");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, routes_hello).await.unwrap();

    Ok(())
}

// First layer (middleware)
async fn main_response_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
