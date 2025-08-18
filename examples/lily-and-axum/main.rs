use axum::{Router, routing::get};

mod types;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = app.merge(types::content::get_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on http://127.0.0.1:3000/");
    axum::serve(listener, app).await.unwrap();
}
