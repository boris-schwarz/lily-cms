//! Provides structured, RFC 7807-compliant error handling for the Axum service.
//!
//! This module defines a two-part error handling system:
//!
//! 1.  The [`Problem`] enum represents high-level, semantic errors that occur
//!     within the application's business logic.
//! 2.  The [`JsonProblem`] struct is the public-facing, serializable representation
//!     of an error, conforming to the "Problem Details for HTTP APIs" standard.
//!
//! Handlers should typically return a [`Problem`], which is then converted into a
//! [`JsonProblem`] before being sent to the client as a response.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// A struct representing an RFC 7807 "Problem Details for HTTP APIs".
///
/// This struct is the serializable format sent to clients when an error occurs.
/// It implements [`IntoResponse`], allowing it to be returned directly from Axum handlers.
///
/// See [RFC 7807](https://tools.ietf.org/html/rfc7807) for more details.
#[derive(Serialize)]
pub struct JsonProblem {
    #[serde(rename = "type", default = "default_type")]
    type_uri: String,
    title: String,
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    detail: String,
}

/// Converts the `JsonProblem` into a well-formed Axum `Response`.
///
/// The response will have the appropriate HTTP status code and a `Content-Type`
/// of `application/problem+json`.
impl IntoResponse for JsonProblem {
    fn into_response(self) -> Response {
        (
            self.status,
            [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
            Json(self),
        )
            .into_response()
    }
}

/// An enum representing all possible high-level application errors.
///
/// This is the primary error type to be used within the application's business
/// logic. It should be converted into a [`JsonProblem`] via [`to_json_problem`]
/// before being sent to the client.
pub enum Problem {
    EndpointNotFound,
    ResourceNotFound { resource: String, id: String },
    InternalError,
}

impl Problem {
    /// Converts a high-level `Problem` into the serializable `JsonProblem`.
    pub fn to_json_problem(self) -> JsonProblem {
        match self {
            Problem::ResourceNotFound { resource, id } => {
                let status_code = StatusCode::NOT_FOUND;
                JsonProblem {
                    type_uri: "/errors/resource-not-found".to_string(),
                    title: status_code
                        .canonical_reason()
                        .unwrap_or(status_code.as_str())
                        .to_owned(),
                    status: status_code,
                    detail: format!(
                        "The resource '{}' with id '{}' was not found.",
                        resource, id
                    ),
                }
            }
            Problem::EndpointNotFound => {
                let status_code = StatusCode::NOT_FOUND;
                JsonProblem {
                    type_uri: "/errors/endpoint-not-found".to_string(),
                    title: status_code
                        .canonical_reason()
                        .unwrap_or(status_code.as_str())
                        .to_owned(),
                    status: status_code,
                    detail: "The endpoint was not found.".to_string(),
                }
            }
            Problem::InternalError => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                JsonProblem {
                    type_uri: "/errors/internal-server-error".to_string(),
                    title: status_code
                        .canonical_reason()
                        .unwrap_or(status_code.as_str())
                        .to_owned(),
                    status: status_code,
                    detail: "An unexpected error occurred on the server.".to_string(),
                }
            }
        }
    }
}
