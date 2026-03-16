pub mod prelude {
    pub use lily_core::Error;
    pub use lily_endpoint::problems::Problem;
    pub use lily_endpoint::responses::ApiResponse;
    pub use lily_endpoint::routing::{
        CreateSingle, DeleteSingle, Endpoint, ReadSingle, RouteBuilder, UpdateSingle,
    };
    pub use lily_macros::endpoint;
}

pub use lily_endpoint::problems::Problem;
pub use lily_endpoint::responses::ApiResponse;
pub use lily_endpoint::routing::{
    CreateSingle, DeleteSingle, Endpoint, ReadSingle, RouteBuilder, UpdateSingle,
};

// pub use axum::{
//     Json, Router,
//     extract::Path,
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{delete, get, patch, post, put},
// };
