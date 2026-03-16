use axum::Router;
use lily::prelude::*;

mod types;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(types::content::Content::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on http://127.0.0.1:3000/content/51de0ea5-635c-4eee-ab70-9827fd14aaca");
    axum::serve(listener, app).await.unwrap();
}
