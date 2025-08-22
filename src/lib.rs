use std::fmt::Debug;
use thiserror::Error;

pub mod prelude;
pub mod problems;
pub mod response;
pub mod types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("example error")]
    Example,
    #[error("unknown error")]
    Unknown,
}
