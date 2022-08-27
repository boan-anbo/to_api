use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Router, routing::{get, post},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::tos::{add_tos, find_tos};

mod controllers;
mod app;
pub mod api_doc;
pub mod api_paths;
pub(crate) mod test_utils;

#[tokio::main]
async fn main() {
    // initialize .env
    dotenv::dotenv().ok();

    // initialize tracing
    // tracing_subscriber::fmt::init();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = app::get_app();


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Textual World!"
}

