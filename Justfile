default:
    just run

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

build: clippy
    cargo build

run: clippy
    cargo run

release: clippy
    cargo build --release

fmt:
    cargo fmt --all

fix:
    cargo clippy --fix --allow-dirty --allow-staged

test:
    cargo test