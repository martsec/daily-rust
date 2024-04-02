#![allow(unused)]
use std::net::SocketAddr;

use axum;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::{response::Html, Router};
use serde::Deserialize;
use tokio;
use tower_http::services::ServeDir;

mod error;
mod web;

pub use crate::error::{Error, Result};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .merge(web::hello_routes())
        // Because we would have a conflict
        .fallback_service(routes_static());

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> LISTENING on {addr}\n");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, routes_hello).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
