pub use lily_cms::prelude::*;

#[endpoint(create_single, read_single, update_single, delete_single)]
pub struct Content {
    title: String,
    body: String,
    summary: Option<String>,
}

impl CreateSingle for Content {
    async fn create_single(payload: &Self::CreatePayload) -> Result<Self, Error> {
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
            return Err(Error::Unknown); // TODO: The user should not have to use lily errors, those are internal only
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

impl UpdateSingle for Content {
    async fn update_single(id: &Self::Id, payload: &Self::UpdatePayload) -> Result<Self, Error> {
        let existing_title = payload.title.clone().unwrap_or("existing title".to_owned());
        if let "invalid" = existing_title.as_str() {
            return Err(Error::Example);
        }
        Ok(Content {
            id: id.clone(),
            title: existing_title,
            body: payload.body.clone().unwrap_or("existing body".to_owned()),
            summary: payload
                .summary
                .clone()
                .or(Some("existing summary".to_owned())),
            created_at: chrono::Utc::now(),
        })
    }
}

impl DeleteSingle for Content {
    async fn delete_single(id: &Self::Id) -> Result<Option<Self>, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown); // TODO: The user should not have to use lily errors, those are internal only
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
