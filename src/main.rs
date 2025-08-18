mod lily;

#[tokio::main]
async fn main() {
    lily::init();
    lily::Cms::new()
        .host("0.0.0.0")
        .port(3000)
        .build()
        .serve()
        .await
        .unwrap();
}
