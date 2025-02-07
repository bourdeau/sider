
start:
  RUST_BACKTRACE=1 cargo run

fmt:
  cargo check
  cargo fmt
  cargo clippy

client:
  nc 127.0.0.1 6379

test:  
  RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

doc:
  cargo doc --no-deps
