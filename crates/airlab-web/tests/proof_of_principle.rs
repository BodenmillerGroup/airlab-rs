use airlab_lib::model;
use airlab_web::web::{ClientError, Error};
use axum::http::StatusCode;

#[test]
fn entity_not_found_error_mapping_proof_of_principle() {
    let err = Error::from(model::Error::EntityNotFound {
        entity: "panel",
        id: 42,
    });

    let (status, client_error) = err.client_status_and_error();

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(matches!(
        client_error,
        ClientError::ENTITY_NOT_FOUND {
            entity: "panel",
            id: 42
        }
    ));
}
