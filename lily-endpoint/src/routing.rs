//! Provides traits and functions for building REST API endpoints from structs

use axum::Router;
use lily_core::Error;
use serde::{Deserialize, Serialize};

pub trait RouteBuilder: Endpoint {
    fn add_create_single_route(router: Router) -> Router;
    fn add_read_single_route(router: Router) -> Router;
    fn add_update_single_route(router: Router) -> Router;
    // fn add_replace_single_route(router: Router) -> Router;
    fn add_delete_single_route(router: Router) -> Router;

    fn routes() -> Router {
        let router: Router = Router::new();
        let router = Self::add_create_single_route(router);
        let router = Self::add_read_single_route(router);
        let router = Self::add_update_single_route(router);
        // let router = Self::add_replace_single_route(router);
        let router = Self::add_delete_single_route(router);
        router
    }
}

/// Defines the contract for a type that can be exposed as a REST API endpoint.
pub trait Endpoint: Serialize + Sized {
    type Id;
    type CreatePayload: for<'de> Deserialize<'de>;
    type UpdatePayload: for<'de> Deserialize<'de>;

    fn get_name() -> String;
    fn get_path() -> String;
    fn get_path_with_id() -> String;
}

#[allow(async_fn_in_trait)]
pub trait CreateSingle: Endpoint {
    async fn create_single(payload: &Self::CreatePayload) -> Result<Self, Error>;
}

#[allow(async_fn_in_trait)]
pub trait ReadSingle: Endpoint {
    async fn read_single(id: &Self::Id) -> Result<Option<Self>, Error>;
}

#[allow(async_fn_in_trait)]
pub trait UpdateSingle: Endpoint {
    async fn update_single(id: &Self::Id, payload: &Self::UpdatePayload) -> Result<Self, Error>;
}

// #[allow(async_fn_in_trait)]
// pub trait ReplaceSingle: Endpoint {
//     async fn replace_single(id: &Self::Id, payload: &Self::UpdatePayload) -> Result<Self, Error>;
// }

#[allow(async_fn_in_trait)]
pub trait DeleteSingle: Endpoint {
    async fn delete_single(id: &Self::Id) -> Result<Option<Self>, Error>;
}
