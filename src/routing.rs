//! Provides traits and functions for building REST API endpoints from structs

use crate::Error;
use axum::Router;
use serde::{Deserialize, Serialize};

pub trait RouteBuilder: Endpoint {
    fn add_create_single_route(router: Router) -> Router;
    fn add_read_single_route(router: Router) -> Router;

    fn routes() -> Router {
        let router: Router = Router::new();
        let router = Self::add_read_single_route(router);
        let router = Self::add_create_single_route(router);
        router
    }
}

/// Defines the contract for a type that can be exposed as a REST API endpoint.
pub trait Endpoint: Serialize + Sized {
    type Id;
    type PostPayload: for<'de> Deserialize<'de>;

    fn get_name() -> String;
    fn get_path() -> String;
    fn get_path_with_id() -> String;
}

/// A capability trait for creating a new resource.
///
/// This trait should be implemented by the resource's business logic layer.
#[allow(async_fn_in_trait)]
pub trait CreateSingle: Endpoint {
    async fn create_single(payload: &Self::PostPayload) -> Result<Self, Error>;
}

#[allow(async_fn_in_trait)]
pub trait ReadSingle: Endpoint {
    async fn read_single(id: &Self::Id) -> Result<Option<Self>, Error>;
}
