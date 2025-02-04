fmt:
    cargo check
    cargo fmt

test:  
  RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

doc:
    cargo doc --no-deps
