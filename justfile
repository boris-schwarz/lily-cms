mod hurl 'tests/hurl.just'

run:
    cargo run

test:
    just hurl test

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
