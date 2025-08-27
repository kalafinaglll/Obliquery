## unittest
```
cargo test -p obq-backend-rss3
cargo test -p obq-backend-rss3 test_rss3_mul -- --nocapture
RUSTFLAGS="-A warnings" cargo test -p obq-backend-rss3 -- --nocapture
```