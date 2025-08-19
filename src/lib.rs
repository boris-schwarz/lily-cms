use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

use crate::problems::Problem;

pub mod problems;
pub mod types;

pub trait Repository<T, U>: Clone + Debug + serde::Serialize {
    fn create_one(payload: U) -> Result<T, Error>;
    fn read_one(id: String) -> Result<T, Error>;
    fn read_all() -> Result<Vec<T>, Error>;
    fn update_one(id: String, payload: U) -> Result<T, Error>;
    fn delete_one(id: String) -> Result<T, Error>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
}

enum ApiResponse<T: Serialize> {
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
            ApiResponse::NotFound(problem) => Json(problem.to_json_problem()).into_response(),
            ApiResponse::Erroneous(problem) => Json(problem.to_json_problem()).into_response(),
        }
    }
}
