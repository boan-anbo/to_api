use axum::Router;
use axum::routing::{get, post};
use utoipa::{
    Modify,
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::tos::{add_tos, find_tos};

pub fn get_app() -> Router {
    #[derive(OpenApi)]
    #[openapi(

    paths(
    crate::controllers::tos::add_tos,
    crate::controllers::tos::find_tos,
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
    pub(crate) struct ApiDoc;
// build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // `GET /` goes to `root`
        .route("/", get(crate::root))
        // `POST /users` goes to `create_user`
        .route("/add_tos", post(add_tos))
        .route("/find_tos", post(find_tos));
    app
}
