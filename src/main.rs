use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::{get, post},
};
use serde::{Deserialize, Serialize};
use to_core::to::to_dto::{TextualObjectAddManyDto, TextualObjectStoredReceipt};
use to_core::to::to_struct::TextualObject;
use to_core::to_machine::to_machine_struct::TextualObjectMachine;
use utoipa::{
    Modify,
    OpenApi, openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use controllers::{add_tos, find_tos};

mod controllers;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(

    paths(
    controllers::add_tos,
    controllers::find_tos,
    ),
    components(
    schemas(
    to_core::to::to_dto::TextualObjectAddManyDto,
    to_core::to::to_dto::TextualObjectAddDto,
    to_core::to::to_dto::TextualObjectStoredReceipt,
    to_core::to::to_dto::TextualObjectFindRequestDto,
    to_core::to::to_dto::TextualObjectFindResultDto,
    )
    ),
    tags(
    (name = "ToApi", description = "Textual Object Api")
    )

    )]
    struct ApiDoc;
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/add_tos", post(add_tos))
        .route("/find_tos", post(find_tos));


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

