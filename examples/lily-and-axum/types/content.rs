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

impl CreateOne<Content> for Content {
    fn create_one(payload: &<Content as Endpoint>::PostPayload) -> Result<Content, Error> {
        Ok(Content {
            id: String::from("insert-uuid-here"),
            title: payload.title.clone(),
            body: payload.body.clone(),
            summary: payload.summary.clone(),
            created_at: chrono::Utc::now(),
        })
    }
}
impl ReadOne<Content> for Content {
    fn read_one(id: &String) -> Result<Option<Content>, Error> {
        if let "invalid" = id.as_str() {
            return Err(Error::Unknown);
        }
        if let "unknown" = id.as_str() {
            return Ok(None);
        }
        Ok(Some(Content::new()))
    }
}

// MARK: Repository
/* impl Repository<Content, PostContent> for Content {
    /*  fn create_one(payload: &PostContent) -> Result<Content, Error> {
        if let "invalid" = payload.body.as_str() {
            return Err(Error::Example);
        }
        Ok(Content {
            id: String::from("11111111-2222-3333-4444-555555555555"),
            title: payload.title.clone(),
            body: payload.body.clone(),
            summary: payload.summary.clone(),
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
    } */

    fn read_all() -> Result<Vec<Content>, Error> {
        Ok(vec![
            Content::new(),
            Content::new(),
            Content::new(),
            Content::new(),
        ])
    }

    fn update_one(id: &String, payload: &PostContent) -> Result<Content, Error> {
        let content: Content = Content {
            id: id.clone(),
            title: payload.title.clone(),
            body: payload.body.clone(),
            summary: payload.summary.clone(),
            created_at: chrono::Utc::now(),
        };
        Ok(content)
    }

    fn delete_one(id: &String) -> Result<(), Error> {
        Ok(())
    }
}
 */
