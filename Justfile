start:
  RUST_BACKTRACE=1 cargo run

fmt:
  cargo check
  cargo fmt
  cargo clippy

client:
  nc 127.0.0.1 6379

test-ut:
  RUST_BACKTRACE=1 cargo test --tests ut

test-func:
  rm -rf ~/.local/share/sider/appendonly.aof
  RUST_BACKTRACE=1 cargo test --tests inte -- --nocapture --test-threads=1

bench:
  cargo bench --bench my_benchmark

bench-graph:
  cargo flamegraph --bench my_benchmark

doc:
  cargo doc --no-deps
