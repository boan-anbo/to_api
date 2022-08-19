use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use to_core::to::to_dto::{TextualObjectAddManyDto, TextualObjectStoredReceipt};
use to_core::to::to_struct::TextualObject;
use to_core::to_machine::to_machine_struct::TextualObjectMachine;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/add_tos", post(add_tos));


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
    "Hello, World!"
}

async fn add_tos(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(_payload): Json<TextualObjectAddManyDto>,
) -> impl IntoResponse {
    // insert your application logic here

    let mut tom = TextualObjectMachine::new_from_dto(
        &_payload ).await;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let receipt = tom.add_tos(_payload).await;
    (StatusCode::CREATED, Json(receipt))
}

#[derive(Serialize, Deserialize)]
struct AddCard {
    db_path: String,
    json: String,
}