pub use lily_cms::prelude::*;

#[expose_struct]
pub struct Content {
    body: String,
    summary: String,
}

// 🐞 this is for debugging only
impl Content {
    fn new() -> Self {
        Content {
            id: String::from("11111111-2222-3333-4444-555555555555"),
            body: String::from("#Doloribus Quia\nTenetur delectus rem:\n- Eveniet\n- Fugiat"),
            summary: String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit."),
            created_at: chrono::Utc::now(),
        }
    }
}

// MARK: Repository
impl Repository<Content, ContentPayload> for Content {
    fn create_one(payload: &ContentPayload) -> Result<Content, Error> {
        if let "invalid" = payload.body.as_str() {
            return Err(Error::Example);
        }
        Ok(Content {
            id: String::from("11111111-2222-3333-4444-555555555555"),
            body: payload.body.clone(),
            summary: payload.body.clone(),
            created_at: chrono::Utc::now(),
        })
    }

    fn read_one(id: &String) -> Result<Option<Content>, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown);
        }
        if let "unknown" = id.as_str() {
            return Ok(None);
        }
        Ok(Some(Content::new()))
    }

    fn read_all() -> Result<Vec<Content>, Error> {
        Ok(vec![
            Content::new(),
            Content::new(),
            Content::new(),
            Content::new(),
        ])
    }

    fn update_one(id: &String, payload: &ContentPayload) -> Result<Content, Error> {
        let content: Content = Content {
            id: id.clone(),
            body: payload.body.clone(),
            summary: payload.body.clone(),
            created_at: chrono::Utc::now(),
        };
        Ok(content)
    }

    fn delete_one(id: &String) -> Result<(), Error> {
        Ok(())
    }
}
