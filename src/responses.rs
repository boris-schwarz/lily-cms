//! Defines the primary `ApiResponse` type used by all Axum handlers.

use crate::problems::Problem;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// A generic enum for all API responses, simplifying handler return types.
///
/// This enum acts as a unified return type for Axum handlers, encapsulating all
/// possible successful and unsuccessful outcomes. It leverages Axum's [`IntoResponse`]
/// trait to automatically map each variant to the correct HTTP status code and
/// response body, reducing boilerplate in the handler functions.
pub enum ApiResponse<T: Serialize> {
    Ok(T),
    Created(T),
    NoContent,
    NotFound(Problem),
    Erroneous(Problem),
    // Escape hatch for special cases, avoid using
    Custom(StatusCode, T),
}

/// Converts the `ApiResponse` into a concrete `axum::response::Response` and
/// maps each variant to the appropriate HTTP status code and body format.
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok(content) => (StatusCode::OK, Json(content)).into_response(),
            ApiResponse::Created(content) => (StatusCode::CREATED, Json(content)).into_response(),
            ApiResponse::NoContent => StatusCode::NO_CONTENT.into_response(),
            ApiResponse::NotFound(problem) => {
                let json_problem = problem.to_json_problem();
                (json_problem.status, Json(json_problem)).into_response()
            }
            ApiResponse::Erroneous(problem) => {
                let json_problem = problem.to_json_problem();
                (json_problem.status, Json(json_problem)).into_response()
            }
            ApiResponse::Custom(status_code, content) => {
                (status_code, Json(content)).into_response()
            }
        }
    }
}
