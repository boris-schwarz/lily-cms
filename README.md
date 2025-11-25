# lily-cms

Lily CMS aims to provide an easy DX (developer experience) to set up a headless CMS in a minute, so more time can be spent working on the actual project.

> Lily CMS is a hobby project I'm working on while learning rust. It is not production-ready nor backed by anyone but my free time.


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

## Run the existing example
`cargo run --example lily-and-axum`

## Check what code is generated for the existing example
`cargo expand --example lily-and-axum > expanded_example.rs`