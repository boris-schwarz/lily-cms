//! Provides traits and functions for building REST API endpoints from structs

use crate::{Error, Problem, responses::ApiResponse};
use axum::{Json, Router, extract::Path};
use std::fmt::Debug;

/// Generates an `axum::Router` for a type that implements the [`Endpoint`] trait.
pub fn get_routes<T: Endpoint>() -> Router
where
    T: Endpoint + Send + 'static,
    <T as Endpoint>::PostPayload: serde::de::DeserializeOwned + Send + 'static,
{
    println!("🟢 The struct's path is: {}", T::get_path());

    let router: Router = Router::new();
    let create_one_router: Router =
        Router::new().route(&T::get_path(), axum::routing::post(T::create_one_handler));
    let read_one_router: Router = Router::new().route(
        &T::get_path_with_id(),
        axum::routing::get(T::read_one_handler),
    );
    let router: Router = router.merge(create_one_router);
    let router: Router = router.merge(read_one_router);
    router
}

/// Defines the contract for a type that can be exposed as a REST API endpoint.
pub trait Endpoint: Clone + Debug + serde::Serialize {
    type PostPayload;
    type PatchPayload;
    fn get_name() -> String;
    fn get_path() -> String;
    fn get_path_with_id() -> String;
    fn create_one_handler(
        payload: Json<Self::PostPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send; // by adding `+ Send` to the return type, we are making it a mandatory part of the Endpoint contract. Now, any type that implements LilyType must provide a create_one_handler function that returns a thread-safe future.
    fn create_many_handler(
        payload: Json<Self::PostPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn read_one_handler(id: Path<String>) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn read_many_handler() -> impl Future<Output = ApiResponse<Vec<Self>>> + Send;
    fn update_one_handler(
        id: Path<String>,
        payload: Json<Self::PatchPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn update_many_handler(
        payload: Json<Self::PatchPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn replace_one_handler(
        id: Path<String>,
        payload: Json<Self::PostPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn replace_many_handler(
        payload: Json<Self::PostPayload>,
    ) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn delete_one_handler(id: Path<String>) -> impl Future<Output = ApiResponse<Self>> + Send;
    fn delete_many_handler(id: Path<String>) -> impl Future<Output = ApiResponse<Self>> + Send;
}

/// A capability trait for creating a new resource.
///
/// This trait should be implemented by the resource's business logic layer.
pub trait CreateOne<T: Endpoint>: Clone + Debug + serde::Serialize {
    fn create_one(payload: &T::PostPayload) -> Result<T, Error>;
}

/// A capability trait for reading a single resource by its ID.
///
/// This trait should be implemented by the resource's business logic layer.
pub trait ReadOne<T>: Clone + Debug + serde::Serialize {
    fn read_one(id: &String) -> Result<Option<T>, Error>;
}

/// A capability trait for reading multiple resources.
///
/// This trait should be implemented by the resource's business logic layer.
pub trait ReadMany<T>: Clone + Debug + serde::Serialize {
    fn read_many() -> Result<Option<T>, Error>;
}
