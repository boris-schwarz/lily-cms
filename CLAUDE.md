# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

```bash
# Build the project
cargo build
cargo build --release

# Run the CMS server
cargo run

# Check code for errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests (when implemented)
cargo test
cargo test -- --nocapture  # Show println! output
```

## Architecture Overview

Lily CMS is a Rust-based content management system built on Axum with a powerful derive macro system for automatic CRUD generation.

### Core Design Patterns

1. **Builder Pattern**: The CMS uses a fluent builder API for configuration:
   ```rust
   lily::Cms::new()
       .host("0.0.0.0")
       .port(3000)
       .build()
       .serve()
   ```

2. **Derive Macro System**: The `GeneratePayload` macro automatically generates:
   - RESTful endpoints (`/content`, `/content/{id}`)
   - Payload structs for API requests (excluding fields marked with `#[metadata]`)
   - CRUD handlers with standard naming (`derived_create_one`, etc.)
   - Route registration via `get_routes()` function

3. **Repository Pattern**: Generic trait-based data access:
   ```rust
   pub trait Repository<T, U> {
       fn create_one(payload: U) -> Result<T, Error>;
       fn read_one(id: String) -> Result<T, Error>;
       fn read_all() -> Result<Vec<T>, Error>;
       fn update_one(id: String, payload: U) -> Result<T, Error>;
       fn delete_one(id: String) -> Result<T, Error>;
   }
   ```

### Adding New Content Types

To add a new content type:
1. Create a struct with the derive macro:
   ```rust
   #[derive(Clone, Debug, serde::Serialize, GeneratePayload)]
   pub struct Article {
       #[metadata]
       pub id: String,
       #[metadata]
       pub created_at: String,
       pub title: String,
       pub content: String,
   }
   ```
2. Implement the `Repository` trait for your type
3. Add an `invalid()` method returning an error instance
4. Routes are automatically generated and can be merged into the main router

### Module Structure

- `src/lily/core/`: Core functionality (builder, environment config, errors)
- `src/lily/types/`: Domain types and their implementations
- `lily-cms-derive/`: Procedural macro for code generation

### Environment Configuration

The `FromEnv` trait provides type-safe environment variable reading with fallbacks:
- `HOST`: Server host (default: "127.0.0.1")
- `PORT`: Server port (default: 3000)
- Additional config can be added by implementing `FromEnv` for new types

### Key Implementation Notes

- The derive macro uses syn and quote for AST manipulation
- Route paths are automatically converted to snake_case
- Error handling uses a custom `Error` enum
- All CRUD operations follow consistent naming patterns
- The macro generates a companion `{Type}Payload` struct for API requests