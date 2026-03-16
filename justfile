mod hurl 'tests/hurl.just'

run:
    cargo run -p example-basic

expand:
    cargo expand -p example-basic > examples/basic/expanded_example_basic.rs

test:
    just hurl test

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
