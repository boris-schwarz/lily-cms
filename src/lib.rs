use std::fmt::Debug;
use thiserror::Error;

pub mod integrations;
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
