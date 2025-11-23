use serde::{Deserialize, Serialize};

use crate::ApiResponse;

pub trait HttpGetSingleHandler {
    type Id;
    type Output: Serialize;

    async fn get_single(&self, id: Self::Id) -> ApiResponse<Self::Output>;
}
pub trait HttpPostHandler {
    type Input: for<'de> Deserialize<'de>;
    type Output: Serialize;

    async fn create(&self, payload: Self::Input) -> ApiResponse<Self::Output>;
}
