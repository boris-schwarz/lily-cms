use thiserror::Error;

/// The primary error type for this library.
///
/// This enum aggregates all possible error conditions that can occur during
/// the execution of library functions.
#[derive(Error, Debug)]
pub enum Error {
    #[error("This is an example error.")]
    Example,
    #[error("An unknown error occured.")]
    Unknown,
}
