pub use lily_cms::types::prelude::*;

// MARK: Type
#[derive(Clone, Debug, serde::Serialize, GeneratePayload)]
pub struct Content {
    #[metadata]
    id: String,
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
        }
    }
    fn invalid() -> Self {
        Content {
            id: String::from(""),
            body: String::from(""),
            summary: String::from(""),
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

// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------
// -----------------------------------

// generate the From implementation for the type and it's generated payload
impl From<ContentPayload> for Content {
    fn from(payload: ContentPayload) -> Self {
        let mut entity = Content {
            //id: uuid::Uuid::new_v4().to_string(),
            id: String::from("uuid-from-the-macro"), // TODO
            body: payload.body,
            summary: payload.summary,
        };

        entity
    }
}

// generate routes
pub fn get_routes() -> Router {
    Router::new()
        .route("/content", post(derived_create_one))
        .route("/content/{id}", get(derived_read_one))
        .route("/content", get(derived_read_all))
        .route("/content/{id}", put(derived_update_one))
        .route("/content/{id}", delete(derived_delete_one))
}

// MARK: CREATE

// generate "create one"
pub async fn derived_create_one(
    Json(payload): Json<ContentPayload>,
) -> (StatusCode, Json<Content>) {
    let r: Result<Content, Error> = Content::create_one(payload);

    match r {
        Ok(data) => (StatusCode::CREATED, Json(data)),
        Err(error_msg) => {
            eprintln!(concat!("Error creating one [content]: {}"), error_msg);
            (StatusCode::NOT_FOUND, Json(Content::invalid()))
        }
    }
}

// MARK: READ

// generate "read one"
pub async fn derived_read_one(Path(id): Path<String>) -> (StatusCode, Json<Content>) {
    let r: Result<Content, Error> = Content::read_one(id);

    match r {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(error_msg) => {
            eprintln!(concat!("Error fetching one [content]: {}"), error_msg);
            (StatusCode::NOT_FOUND, Json(Content::invalid()))
        }
    }
}

// generate "read all"
pub async fn derived_read_all() -> (StatusCode, Json<Vec<Content>>) {
    let r: Result<Vec<Content>, Error> = Content::read_all();

    match r {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(error_msg) => {
            eprintln!(concat!("Error fetching all [content]: {}"), error_msg);
            (StatusCode::NOT_FOUND, Json(Vec::new()))
        }
    }
}

// MARK: UPDATE

// generate "update one"
pub async fn derived_update_one(
    Path(id): Path<String>,
    Json(payload): Json<ContentPayload>,
) -> (StatusCode, Json<Content>) {
    let r: Result<Content, Error> = Content::update_one(id, payload);

    match r {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(error_msg) => {
            eprintln!(concat!("Error updating one [content]: {}"), error_msg);
            (StatusCode::NOT_FOUND, Json(Content::invalid()))
        }
    }
}

// MARK: DELETE

// generate "delete one"
pub async fn derived_delete_one(Path(id): Path<String>) -> (StatusCode, Json<Content>) {
    let r: Result<Content, Error> = Content::delete_one(id);

    match r {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(error_msg) => {
            eprintln!(concat!("Error deleting one [content]: {}"), error_msg);
            (StatusCode::NOT_FOUND, Json(Content::invalid()))
        }
    }
}
