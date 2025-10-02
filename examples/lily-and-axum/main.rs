use lily_cms::prelude::*;

mod types;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = app.merge(get_routes::<types::content::Content>());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on http://127.0.0.1:3000/");
    axum::serve(listener, app).await.unwrap();
}
