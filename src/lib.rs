pub mod errors;
pub mod problems;
pub mod responses;
pub mod routing;
pub mod types;
pub mod prelude {
    pub use crate::Error;
    pub use crate::problems::Problem;
    pub use crate::responses::ApiResponse;
    pub use crate::routing::{CreateOne, Endpoint, ReadOne, get_routes};
    pub use crate::types::Repository;
    pub use axum::{
        Json, Router,
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        routing::{delete, get, patch, post, put},
    };
    pub use lily_macros::endpoint;
}

pub use errors::Error;
pub use problems::Problem;
pub use responses::ApiResponse;
pub use routing::{CreateOne, Endpoint, ReadOne, get_routes};
