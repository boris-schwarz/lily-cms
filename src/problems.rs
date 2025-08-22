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
    pub status: StatusCode,
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
