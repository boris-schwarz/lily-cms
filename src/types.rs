use crate::Error;
use std::fmt::Debug;

pub trait Repository<T, U>: Clone + Debug + serde::Serialize {
    fn create_one(payload: &U) -> Result<T, Error>;
    fn read_one(id: &String) -> Result<Option<T>, Error>;
    fn read_all() -> Result<Vec<T>, Error>;
    fn update_one(id: &String, payload: &U) -> Result<T, Error>;
    fn delete_one(id: &String) -> Result<(), Error>;
}
