pub use lily_cms::prelude::*;

#[endpoint(read_single, create_single)]
pub struct Content {
    title: String,
    body: String,
    summary: Option<String>,
}

impl CreateSingle for Content {
    async fn create_single(payload: &Self::PostPayload) -> Result<Self, Error> {
        if let "invalid" = payload.body.as_str() {
            return Err(Error::Example);
        }
        Ok(Content {
            id: String::from("uuid-from-database"),
            title: payload.title.clone(),
            body: payload.body.clone(),
            summary: payload.summary.clone(),
            created_at: chrono::Utc::now(),
        })
    }
}

impl ReadSingle for Content {
    async fn read_single(id: &Self::Id) -> Result<Option<Self>, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown);
        }
        if let "unknown" = id.as_str() {
            return Ok(None);
        }
        Ok(Some(Content {
            id: id.to_owned(),
            title: String::from("Lorem Ipsum"),
            body: String::from("#Doloribus Quia\nTenetur delectus rem:\n- Eveniet\n- Fugiat"),
            summary: Some(String::from(
                "Lorem ipsum dolor sit amet consectetur adipisicing elit.",
            )),
            created_at: chrono::Utc::now(),
        }))
    }
}
