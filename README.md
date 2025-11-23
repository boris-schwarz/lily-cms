# lily-cms

## Quick example
1. Add the endpoint macro to your struct
```rust
#[endpoint]
pub struct Content {
    title: String,
    body: String,
    summary: Option<String>,
}
```
2. Add the routes for your type to the axum router
```rust
let app = Router::new().route("/", get(|| async { "Hello, World!" }));
let app = app.merge(get_routes::<Content>());
```

## Run
cargo run --example lily-and-axum

## Debug Macro Generation
cargo expand --example lily-and-axum > expanded_example.rs