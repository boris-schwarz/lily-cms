use crate::problems::Problem;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum ApiResponse<T: Serialize> {
    Ok(T),
    Created(T),
    NoContent,
    NotFound(Problem),
    Erroneous(Problem),
}

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
        }
    }
}
