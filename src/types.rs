use crate::Error;
use std::fmt::Debug;

pub trait Repository<T, U>: Clone + Debug + serde::Serialize {
    fn create_one(payload: &U) -> Result<T, Error>;
    fn read_one(id: &String) -> Result<Option<T>, Error>;
    fn read_all() -> Result<Vec<T>, Error>;
    fn update_one(id: &String, payload: &U) -> Result<T, Error>;
    fn delete_one(id: &String) -> Result<(), Error>;
}

pub trait GetOne<T>: Clone + Debug + serde::Serialize {
    fn get_one(id: &String) -> Result<Option<T>, Error>;
}

// read_one     GET
// read_many    GET
// create_one   POST
// create_many  POST
// replace_one  PUT
// replace_many PUT
// update_one   PATCH
// update_many  PATCH
// delete_one   DELETE
// delete_many  DELETE
