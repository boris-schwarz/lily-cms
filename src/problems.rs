use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonProblem {
    #[serde(rename = "type", default = "default_type")]
    type_uri: String,
    title: String,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    detail: String,
}

/* fn default_type() -> String {
    "about:blank".to_string()
} */

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

pub enum Problem {
    ResourceNotFound { resource: String, id: String },
    InternalError,
}

impl Problem {
    pub fn to_json_problem(self) -> JsonProblem {
        match self {
            Problem::ResourceNotFound { resource, id } => JsonProblem {
                type_uri: "/errors/resource-not-found".to_string(),
                title: "Resource Not Found".to_string(),
                status: StatusCode::NOT_FOUND,
                detail: format!(
                    "The resource '{}' with id '{}' was not found.",
                    resource, id
                ),
            },
            Problem::InternalError => JsonProblem {
                type_uri: "/errors/internal-server-error".to_string(),
                title: "Internal Server Error".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
                detail: "An unexpected error occurred on the server.".to_string(),
            },
        }
    }
}
