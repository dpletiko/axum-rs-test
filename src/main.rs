//! Run with
//!
//! ```not_rust
//! cargo run -p axum-rs-test
//! ```
//!
//! Checkout the [diesel webpage](https://diesel.rs) for
//! longer guides about diesel
//!
//! Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
//! for a real world application using axum and diesel

//& #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate diesel_migrations;

use axum::{
    // extract::State,
    http::StatusCode,
    routing::get,
    Router,
};
// use diesel::prelude::*;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
use tracing::Level;
use tower_http::trace::{self, TraceLayer};


mod utils;

mod error_handler;
mod db;
mod schema;

mod auth;
mod users;
mod boards;
mod widgets;


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    db::init().await;


    // build our application with some routes
    let app = Router::new()
        .route("/", get(root))
        .merge(auth::init_routes())
        .merge(users::init_routes())
        .merge(boards::init_routes())
        .merge(widgets::init_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(tower_livereload::LiveReloadLayer::new());
        // .with_state();


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
