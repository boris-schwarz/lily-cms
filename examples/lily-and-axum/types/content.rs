pub use lily_cms::prelude::*;

#[endpoint(read_one, create_one)]
pub struct Content {
    title: String,
    body: String,
    summary: Option<String>,
}

// 🐞 this is for debugging only
impl Content {
    fn new() -> Self {
        Content {
            id: String::from("11111111-2222-3333-4444-555555555555"),
            title: String::from("Lorem Ipsum"),
            body: String::from("#Doloribus Quia\nTenetur delectus rem:\n- Eveniet\n- Fugiat"),
            summary: Some(String::from(
                "Lorem ipsum dolor sit amet consectetur adipisicing elit.",
            )),
            created_at: chrono::Utc::now(),
        }
    }
}

impl GetSingle for Content {
    async fn get_single(id: &Self::Id) -> Result<Option<Self>, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown);
        }
        if let "unknown" = id.as_str() {
            return Ok(None);
        }
        Ok(Some(Content::new()))
    }
}
impl CreateSingle for Content {
    async fn create_single(payload: &Self::PostPayload) -> Result<Self, Error> {
        Ok(Content {
            id: String::from("uuid-from-database"),
            title: payload.title.clone(),
            body: payload.body.clone(),
            summary: payload.summary.clone(),
            created_at: chrono::Utc::now(),
        })
    }
}
