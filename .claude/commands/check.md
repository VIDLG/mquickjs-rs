Run full CI checks locally: format, lint, test, and build release. Report any failures.

```
cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo build --release
```
