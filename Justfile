# =============================================
# Justfile - Rust Power User
# =============================================

# Default command
default:
    just --list

# === Build & Run ===

run: clippy
    cargo run

run-release: clippy
    cargo run --release

build: clippy
    cargo build

build-release: clippy
    cargo build --release

# === Quality & Linting ===

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

fix:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

# === Testing ===

test:
    cargo nextest run

test-verbose:
    cargo nextest run --verbose

test-all: test
    cargo test --doc

# === Audit & Security ===

audit:
    cargo audit

outdated:
    cargo outdated

udeps:
    cargo udeps --all-targets

# === Code Analysis ===

expand *ARGS:
    cargo expand {{ARGS}}

flamegraph *ARGS:
    cargo flamegraph {{ARGS}}

llvm-lines:
    cargo llvm-lines --release | head -n 30

tarpaulin:
    cargo tarpaulin --ignore-tests --out Html

# === Formatting & Dependencies ===

fmt-check:
    cargo fmt --all -- --check

update:
    cargo update

add *ARGS:
    cargo add {{ARGS}}

rm *ARGS:
    cargo remove {{ARGS}}

# === Release & Publishing ===

semver-check:
    cargo semver-checks check-release

publish-dry:
    cargo publish --dry-run

# === Utils ===

watch +ARGS="run":
    cargo watch -x "{{ARGS}}"

clean:
    cargo clean

size:
    cargo size --release

# === Full Check ===
check-all: clippy test fmt-check audit outdated
    @echo "✅ All checks passed!"