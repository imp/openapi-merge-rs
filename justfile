
build:
    cargo build --workspace --all-targets
clean:
    cargo clean
    cargo test --workspace

clippy:
    cargo clippy --workspace --all-targets
c:
    cargo c
pedantic:
    cargo clippy --workspace --all-targets --features pedantic
update:
    cargo update
bloat:
    cargo bloat
cbuild: clean build
rustfmt:
    cargo fmt --all -- --check
alias fmt := rustfmt
check: rustfmt update test clippy
test:
    cargo test --workspace
fixlock:
    rm Cargo.lock
    cargo update
    git add Cargo.lock
