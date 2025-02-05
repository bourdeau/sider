fmt:
    cargo check
    cargo fmt
    cargo clippy

test:  
  RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

doc:
    cargo doc --no-deps
