pub mod errors;
pub mod problems;
pub mod responses;
pub mod routing;
pub mod prelude {
    pub use crate::Error;
    pub use crate::problems::Problem;
    pub use crate::responses::ApiResponse;
    pub use crate::routing::{CreateSingle, Endpoint, ReadSingle, RouteBuilder, get_routes};
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
pub use routing::{CreateSingle, Endpoint, ReadSingle, RouteBuilder, get_routes};
