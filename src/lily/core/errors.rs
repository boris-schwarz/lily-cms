use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ExampleError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExampleError => write!(f, "This is an example error"),
        }
    }
}
