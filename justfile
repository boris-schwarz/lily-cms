mod hurl 'tests/hurl.just'

run:
    cargo run --example basic

test:
    just hurl test

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
