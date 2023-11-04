#![allow(unused)]

use std::net::SocketAddr;

use axum::middleware::map_response;
use axum::{Router, middleware};
use axum::extract::{Query, Path};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

use crate::model::ModelController;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    // Initialize the modelcontroller
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routers_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr  = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> listening on {addr}");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routers_hello() -> Router {
    Router::new().route(
        "/hello",
        get(handler_hello),
    ).route("/hello2/:name", get(hanlder_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - hanlder_hello", "HANLDER");

    let name = params.name.as_deref().unwrap_or("World");
     Html(format!("Hello <strong>{name}!!!</strong>"))
}

// do the hello2/:name so it return the Hello :name!!! to the browser.

async fn hanlder_hello2(Path(name): Path<String>) -> impl IntoResponse {

    println!("-->> {:<12} - handler_hello2 - {name:?}", "HANLDER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

