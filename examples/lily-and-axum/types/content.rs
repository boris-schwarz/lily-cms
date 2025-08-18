pub use lily_cms::types::prelude::*;

#[lily_type]
pub struct Content {
    body: String,
    summary: String,
}

// 🐞 this is for debugging only
impl Content {
    fn new() -> Self {
        Content {
            id: String::from("86e4fc36-7ce7-4d22-b5a0-5bd819804cce"),
            body: String::from("#header\nthis is some content\n- check this\n- out"),
            summary: String::from("really interesting fact about something"),
            created_at: chrono::Utc::now(),
        }
    }
    fn invalid() -> Self {
        Content {
            id: String::from(""),
            body: String::from(""),
            summary: String::from(""),
            created_at: chrono::Utc::now(),
        }
    }
}

// MARK: Repository
impl Repository<Content, ContentPayload> for Content {
    fn create_one(payload: ContentPayload) -> Result<Content, Error> {
        Ok(payload.into())
    }

    fn read_one(id: String) -> Result<Content, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown);
        }
        Ok(Content::new())
    }

    fn read_all() -> Result<Vec<Content>, Error> {
        if 1 == 2 {
            return Err(Error::Unknown);
        }
        Ok(vec![
            Content::new(),
            Content::new(),
            Content::new(),
            Content::new(),
        ])
    }

    fn update_one(id: String, payload: ContentPayload) -> Result<Content, Error> {
        let mut content: Content = payload.into();
        content.id = id;
        Ok(content)
    }

    fn delete_one(id: String) -> Result<Content, Error> {
        println!("deleting {id}");
        Ok(Content::new())
    }
}
