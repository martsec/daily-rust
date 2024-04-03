#![allow(unused)]
use std::net::SocketAddr;

use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get_service;
use axum::{self, middleware, Json};
use axum::{response::Html, Router};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use tokio;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use crate::error::{Error, Result};

use crate::log::log_request;
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
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("  --> client_error_body {client_error_body}");

            // Build new response
            (*status_code, Json(client_error_body)).into_response()
        });

    // TODO build and log the server log line
    let client_error = client_status_error.unzip().1; // Get the option of
                                                      // client error check out the types
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();

    // We are doing it the other way around. If no error, return response.
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
