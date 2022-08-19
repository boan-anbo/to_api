use axum::Json;
use to_core::to::to_dto::{TextualObjectAddManyDto, TextualObjectFindRequestDto};
use to_core::to_machine::to_machine_struct::TextualObjectMachine;

use crate::{IntoResponse, StatusCode};

#[utoipa::path(
post,
path = "/find_tos",
request_body = TextualObjectFindRequestDto,
responses(
(status = 201, description = "Tos item created successfully", body =  TextualObjectFindResultDto),
)
)]
pub(crate) async fn find_tos(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(_payload): Json<TextualObjectFindRequestDto>,
) -> impl IntoResponse {
    // insert your application logic here

    let mut tom = TextualObjectMachine::new_from_find_dto(
        &_payload).await;



    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let result = tom.find_tos_by_ticket_ids(&_payload).await;
    (StatusCode::CREATED, Json(result))
}

#[utoipa::path(
post,
path = "/add_tos",
request_body = TextualObjectAddManyDto,
responses(
(status = 201, description = "Tos item created successfully", body = TextualObjectStoredReceipt),
)
)]
pub(crate) async fn add_tos(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(_payload): Json<TextualObjectAddManyDto>,
) -> impl IntoResponse {
    // insert your application logic here

    let mut tom = TextualObjectMachine::new_from_add_dto(
        &_payload).await;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let receipt = tom.add_tos(_payload).await;
    (StatusCode::CREATED, Json(receipt))
}
